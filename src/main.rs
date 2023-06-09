use clap::Parser;
use krabligraphy::{emojify, spoilerify};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
struct Args {
    #[clap(short, long, default_value = "")]
    spoilerify: String,

    #[clap(short, long, default_value = "")]
    emojify: String,

    #[clap(short, long, default_value = None)]
    cheer_emoji: Option<String>,
}

fn main() {
    let args = Args::parse();
    if !args.emojify.is_empty() {
        println!("{}", emojify(&args.emojify, args.cheer_emoji));
    }
    if !args.spoilerify.is_empty() {
        println!("{}", spoilerify(&args.spoilerify));
    }
}
