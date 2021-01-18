# sdfgen

SDFGen generates a (preferably) lower resolution distance field from an input image.

This distance field can be upscaled fast with bilinear filters built into gpus nowadays.
Final sharp output of this upscale can be extracted using a threshold filter. With the default parameters in this program, threshold should be set to about 99%.

## Building the project
```console
cargo build --release
```
Without the release flag, the execution time can be up to 10 times slower.
## Running the project
```console
  ./target/release/sdfgen [OPTIONS]
```
```rust
Optional arguments:
  -h,--help             Show this help message and exit
  -v,--verbose          Be verbose
  -f,--file FILE        filepath of the input image. Defaults to
                        "images/sdf500.png"
  -s,--scale SCALE      Scale of the distance field. For example resolution
                        500x500 becomes 250x250 with scale 2 and 125x125 with
                        scale 4. Defaults to 32.
  -t,--threads THREADS  Number of assigned threads. Defaults to 8.
  -V,--version          Show version
```
<!-- Please do not edit the README.md file directly, as it is compiled from the documentation at src/main.rs -->
<!-- Instead, make the edits into the main.rs file and then run the compile_readme.sh or equivalent. -->

License: MIT
