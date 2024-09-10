mod opts;

use crate::opts::{Opts, Person, SubCommand};
use clap::Parser;
use csv::Reader;
use std::fs;

/// rust-li csv -i input.csv -o output.json -d ","
fn main() -> anyhow::Result<()> {
    let opts = Opts::parse();
    match opts.cmd {
        SubCommand::Csv(opts) => {
            let mut reader = Reader::from_path(opts.input)?;

            let mut persons: Vec<Person> = Vec::new();
            // 读取csv文件
            for record in reader.deserialize() {
                let person: Person = record?;
                persons.push(person);
            }

            // 输出json文件
            let result = serde_json::to_string_pretty(&persons)?;
            Ok(fs::write(opts.output, result)?)
        }
    }
}
