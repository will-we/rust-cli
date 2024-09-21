use base64::prelude::BASE64_STANDARD_NO_PAD;
use base64::Engine;
use clap::Parser;
use csv::Reader;
use rand::Rng;
use rust_cli::base64::Base64SubCommand;
use rust_cli::opts::{Opts, SubCommand};
use serde_json::Value;
use std::fs;
use std::io::stdin;

/// rust-li csv -i input.csv -o output.json -d ","
fn main() -> anyhow::Result<()> {
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
                //打印输入的字符串
                print!("Input: {:?}", std_in_str);
                //编码
                let mut buf = String::new();
                BASE64_STANDARD_NO_PAD.encode_string(std_in_str, &mut buf);
                println!("Encoded: {}", buf);
                Ok(())
            }
            Base64SubCommand::Decode(opts) => {
                println!("Decoding: {:?}", opts);
                Ok(())
            }
        },
    }
}

fn generate_password(length: u8) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}
