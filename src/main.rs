mod lib;

use clap::Parser;

/// Simple image processing application.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Image manipulation method which will be applied
    #[clap(short, long)]
    method: String,

    /// Input file path
    #[clap(short, long)]
    infile: Option<String>,

    /// Output file path
    #[clap(short, long)]
    outfile: String,
}

fn main() {
    let arguments = Args::parse();

    match arguments.method.as_str() {
        "blur" => {
            lib::blur(arguments.infile.unwrap(), arguments.outfile);
        }
        "grayscale" => {
            lib::grayscale(arguments.infile.unwrap(), arguments.outfile);
        }
        "fractal" => {
            lib::fractal(arguments.outfile);
        }
        _ => {
            println!("Couldn't find method. Exiting..");
            std::process::exit(-1);
        }
    }
}
