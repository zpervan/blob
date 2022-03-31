mod algorithms;
mod image_processing;
mod constants;

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
            image_processing::pixel::blur(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "brighten" => {
            image_processing::pixel::brighten(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "crop" => {
            image_processing::image::crop(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "rotate" => {
            image_processing::image::rotate(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "invert" => {
            image_processing::pixel::invert(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "grayscale" => {
            image_processing::pixel::grayscale(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "generate" => {
            image_processing::generators::generate(arguments.outfile.unwrap());
        }
        "median" => {
            image_processing::filters::median_filter(arguments.infile.unwrap(), arguments.outfile.unwrap());
        }
        "fractal" => {
            image_processing::generators::fractal(arguments.outfile.unwrap());
        }
        _ => {
            println!("Couldn't find method. Exiting..");
            std::process::exit(-1);
        }
    }
}
