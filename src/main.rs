mod opts;

use crate::opts::{Opts, SubCommand};
use clap::Parser;
use csv::Reader;
use rand::Rng;
use serde_json::Value;
use std::fs;

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
    }
}

fn generate_password(length: u8) -> String {
    rand::thread_rng()
        .sample_iter(&rand::distributions::Alphanumeric)
        .take(length as usize)
        .map(char::from)
        .collect()
}
