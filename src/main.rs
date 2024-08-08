use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "rustCli", version, about, long_about = None)]
struct Opts {
    #[command(subcommand)]
    cmd: SubCommand,
}

#[derive(Parser, Debug)]
enum SubCommand {
    #[command(name = "csv", about = "Subcommand for CSV operations")]
    Csv(CsvOpts),
}

#[derive(Parser, Debug)]
struct CsvOpts {
    /// Name of the person to greet
    #[arg(short, long)]
    input: String,

    /// Output file name
    #[arg(short, long, default_value = "output.json")]
    output: String,

    /// Delimiter to use for output file
    #[arg(short, long, default_value = ",")]
    delimiter: String,

    #[arg(long, default_value_t = true)]
    header: bool,
}

/// rustCli csv -i input.csv -o output.json -d ","
fn main() {
    let opts = Opts::parse();
    println!("{:?}", opts);
}
