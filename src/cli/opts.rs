use crate::base64::Base64SubCommand;
use crate::csv::CsvOpts;
use crate::genpass::GenPassOpts;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rust-cli", version, about="rust命令行工具", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "读取csv文件并默认输出json文件")]
    Csv(CsvOpts),
    #[command(name = "gen-pass", about = "生成随机密码")]
    GenPass(GenPassOpts),
    #[command(subcommand, about = "base64编解码")]
    Base64(Base64SubCommand),
}
