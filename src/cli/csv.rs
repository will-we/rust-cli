use clap::Parser;

use super::validate_path;

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
