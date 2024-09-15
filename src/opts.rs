use clap::Parser;
use std::path;

#[derive(Parser, Debug)]
#[command(name = "rust-cli", version, about="rust命令行工具", long_about = None)]
pub struct Opts {
    #[command(subcommand)]
    pub(crate) cmd: SubCommand,
}

#[derive(Parser, Debug)]
pub enum SubCommand {
    #[command(name = "csv", about = "读取csv文件并默认输出json文件")]
    Csv(CsvOpts),
    #[command(name = "gen-pass", about = "生成随机密码")]
    GenPass(GenPassOpts),
}

#[derive(Debug, Parser)]
pub struct GenPassOpts {
    #[arg(short, long, default_value_t = 16)]
    pub length: u8,

    #[arg(long, default_value_t = true)]
    pub uppercase: bool,

    #[arg(long, default_value_t = true)]
    pub lowercase: bool,

    #[arg(long, default_value_t = true)]
    pub number: bool,

    #[arg(long, default_value_t = true)]
    pub symbol: bool,
}

#[derive(Parser, Debug)]
pub struct CsvOpts {
    /// 文件输入的路径，必填
    #[arg(short, long, required = true, value_parser = validate_path)]
    pub input: String,

    /// 文件输入的路径，默认 output.json
    #[arg(short, long, default_value = "output.json")]
    pub output: String,

    /// 文件分隔符，默认","
    #[arg(short, long, default_value = ",")]
    pub delimiter: String,

    /// 是否有标题行，默认true
    #[arg(long, default_value_t = true)]
    pub header: bool,
}

/// 自定义参数校验器: 校验文件路径是否存在
fn validate_path(path: &str) -> Result<String, String> {
    if path::Path::new(path).exists() {
        Ok(path.to_string())
    } else {
        Err(format!("{} 文件不存在", path))
    }
}
