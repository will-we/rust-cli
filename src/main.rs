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
use tracing::info;
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
                process_http_requests(opts.port, opts.path)
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

async fn process_http_requests(port: u16, path: String) -> anyhow::Result<()> {
    info!("Serving HTTP requests on port {}, path {}", port, path);
    let app = Router::new().route("/", get(index_handler()));
    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    info!("Listening on http://{}", listener.local_addr()?);
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

fn index_handler() -> &'static str {
    "hello world, this is from rust server!"
}
