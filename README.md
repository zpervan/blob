# Image Processing Exercise #
[![Ubuntu](https://github.com/zpervan/image_processing_exercise/actions/workflows/ci.yml/badge.svg)](https://github.com/zpervan/image_processing_exercise/actions/workflows/ci.yml)

This repository contains image processing libraries which can be called from the terminal.

## Setup ## 

### Ubuntu ###
Install the following libraries to run the project:
```shell
> sudo apt-get install libglib2.0-dev libgtk-3-dev
```

## Build ##

Position your terminal in the root of this project and run:
```shell
> cargo build            # Builds in debug mode
> cargo build --release  # Builds in release mode
```

## Run ##

Via executable:
```shell
./image_processing_exercise/target/release/image_processing_exercise -m "median" -i "~/image_processing_exercise/assets/s_n_p_1.png" -o "~/image_processing_exercise/assets/result.png"
```
Or `cargo` command:
```shell
~/image_processing_exercise> cargo run -- -m "median" -i "~/image_processing_exercise/assets/s_n_p_1.png" -o "~/image_processing_exercise/assets/result.png"
```

List of available options:
```shell
-h, --help                 Print help information
-i, --infile <INFILE>      Input file path
-m, --method <METHOD>      Image manipulation method which will be applied
-o, --outfile <OUTFILE>    Output file path
-V, --version              Print version information
```

List of available methods:
* `blur`
* `brighten`
* `crop`
* `rotate`
* `invert`
* `grayscale`
* `generate`
* `fractal`
* `median`

