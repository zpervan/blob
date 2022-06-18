# Image Processing Exercise #
[![Ubuntu](https://github.com/zpervan/image_processing_exercise/actions/workflows/ci.yml/badge.svg)](https://github.com/zpervan/image_processing_exercise/actions/workflows/ci.yml)

This repository contains image processing libraries which can be called from the terminal.

![AppExample](assets/app_example.png)

## Setup ## 

### Ubuntu ###
Install the following libraries to run the project:
```shell
> sudo apt-get install libglib2.0-dev libgtk-3-dev
```

## Run ##

`cargo` command:
```shell
~/image_processing_exercise> cargo run --release
```

## Additional ##

List of available methods (for now):
* `blur`
* `brighten`
* `crop`
* `rotate`
* `invert`
* `grayscale`
* `generate`
* `fractal`
* `median`

