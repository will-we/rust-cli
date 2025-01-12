use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use base64::engine::general_purpose::URL_SAFE;
use base64::prelude::BASE64_STANDARD;
use base64::Engine;
use clap::Parser;
use csv::Reader;
use rand::Rng;
use rust_cli::base64::{Base64Format, Base64SubCommand};
use rust_cli::http::HttpCommand;
use rust_cli::opts::{Opts, SubCommand};
use serde_json::Value;
use std::fs;
use std::io::stdin;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use std::sync::Arc;
use tower_http::services::ServeDir;
use tracing::info;
use tracing::log::warn;

/// rust-li csv -i input.csv -o output.json -d ","
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Starting rust-li...");

    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;

            // 读取csv文件
            let mut jsons = Vec::new();
            // 这里由于reader导致了两次应用
            let headers = reader.headers()?.clone();
            for record in reader.records() {
                let item = record?;
                // 转换为小写
                let value = headers
                    .iter()
                    .zip(item.iter().map(|s| s.to_lowercase()))
                    .collect::<Value>();
                jsons.push(value);
            }

            // 输出json文件
            let result = serde_json::to_string_pretty(&jsons)?;
            Ok(fs::write(opts.output, result)?)
        }
        SubCommand::GenPass(opts) => {
            let pass = generate_password(opts.length);
            println!("Password: {}", pass);
            Ok(())
        }
        SubCommand::Base64(base64_sub_command) => match base64_sub_command {
            Base64SubCommand::Encode(opts) => {
                println!("Encoding: {:?}", opts);
                let mut std_in_str = String::new();
                stdin().read_line(&mut std_in_str)?;
                // 优化去掉/r/n
                let input_str = std_in_str.trim_end();
                //打印输入的字符串
                print!("Input: {:?} ", input_str);
                //编码
                let mut buf = String::new();
                match opts.format {
                    Base64Format::Standard => {
                        BASE64_STANDARD.encode_string(input_str, &mut buf);
                    }
                    Base64Format::UrlSafe => {
                        URL_SAFE.encode_string(input_str, &mut buf);
                    }
                }
                println!("Encoded: {}", buf);
                Ok(())
            }
            Base64SubCommand::Decode(opts) => {
                println!("Decoding: {:?}", opts);
                let mut std_in_str = String::new();
                stdin().read_line(&mut std_in_str)?;
                // 优化去掉/r/n
                let input_str = std_in_str.trim_end();
                //打印输入的字符串
                print!("Input: {:?} ", input_str);
                //编码
                let decoded = match opts.format {
                    Base64Format::Standard => BASE64_STANDARD.decode(input_str)?,
                    Base64Format::UrlSafe => URL_SAFE.decode(input_str)?,
                };
                let decoded_str = String::from_utf8(decoded)?;
                println!("Decoded: {}", decoded_str);
                Ok(())
            }
        },

        SubCommand::Http(http_command) => match http_command {
            HttpCommand::Serve(opts) => {
                process_http_requests(opts.port, opts.path.parse()?)
                    .await
                    .expect("server start failed");
                Ok(())
            }
            HttpCommand::Client(opts) => {
                println!("Client requests to {}", opts.server);
                Ok(())
            }
        },
    }
}

#[derive(Debug)]
struct HttpServeState {
    path: PathBuf,
}

async fn process_http_requests(port: u16, path: PathBuf) -> anyhow::Result<()> {
    info!(
        "Serving HTTP requests on port {}, path {}",
        port,
        path.display()
    );
    let state = HttpServeState { path: path.clone() };
    let dir = ServeDir::new(path)
        .append_index_html_on_directories(true)
        .precompressed_gzip()
        .precompressed_br()
        .precompressed_deflate()
        .precompressed_zstd();
    let app = Router::new()
        .route("/*path", get(file_handler))
        .nest_service("/tower", dir)
        .with_state(Arc::new(state));
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    //noinspection HttpUrlsUsage
    info!("Listening on https://{}", listener.local_addr()?);
    axum::serve(listener, app).await?;
    Ok(())
}

fn generate_password(length: u8) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}

async fn file_handler(
    State(state): State<Arc<HttpServeState>>,
    Path(path): Path<String>,
) -> (StatusCode, String) {
    let p = state.path.join(path);
    info!("Handling file: {}", p.display());
    if !p.exists() {
        (StatusCode::NOT_FOUND, "File already exists".to_owned())
    } else {
        match tokio::fs::read_to_string(p).await {
            Ok(content) => {
                info!("Read {} bytes", content.len());
                (StatusCode::OK, content)
            }
            Err(e) => {
                warn!("Error reading file: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, e.to_string())
            }
        }
    }
}
