use clap::Parser;

#[derive(Parser, Debug)]
#[command(name = "echor")]
#[command(author = "Takkaryx")]
#[command(version = "1.0")]
#[command(about = "rust echo", long_about = None)]
struct Cli {
    #[arg(short = 'n', long, help = "do not print newline")]
    omit_newline: bool,
    #[arg(help = "Input text")]
    text: Vec<String>,
}

fn main() {
    let matches = Cli::parse();
    let ending = if matches.omit_newline { "" } else { "\n" };
    print!("{}{}", matches.text.join(" "), ending);
}
