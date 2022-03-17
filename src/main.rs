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
    outfile: Option<String>,
}

fn main() {
    let arguments = Args::parse();

    match arguments.method.as_str() {
        "blur" => {
            lib::blur(arguments.infile.unwrap(), arguments.outfile.unwrap());
        },
        "brighten" => {
            lib::brighten(arguments.infile.unwrap(), arguments.outfile.unwrap());
        },
        "crop" => {
            lib::crop(arguments.infile.unwrap(), arguments.outfile.unwrap());
        },
        "rotate" => {
            lib::rotate(arguments.infile.unwrap(), arguments.outfile.unwrap());
        },
        "invert" => {
            lib::invert(arguments.infile.unwrap(), arguments.outfile.unwrap());
        },
        "grayscale" => {
            lib::grayscale(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "fractal" => {
            lib::fractal(arguments.outfile.unwrap());
        }
        _ => {
            println!("Couldn't find method. Exiting..");
            std::process::exit(-1);
        }
    }
}
