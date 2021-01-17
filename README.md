# sdf

SDFGen generates a preferably lower resolution distance field of an input image.

This distance field can be upscaled fast with bilinear filters built into gpus nowadays.
Final sharp output of this upscale can be extracted using a threshold filter. With the default parameters in this program, threshold should be set to about 99%.

## Examples
Run the project
```rust
cargo run --release
```
Without the release flag, the execution time can be up to 10 times slower.

Currently all variables are hardcoded, they can be changed in the main program file "/scr/main.rs".
