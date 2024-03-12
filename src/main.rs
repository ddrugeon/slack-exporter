use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about)]
/// Rust version of `echo`
struct Args {
    /// Input text
    #[arg(required(true))]
    text: Vec<String>,
}

fn main() {
    let args = Args::parse();
    println!(
        "{}",
        args.text.join(" "),
    );
}
