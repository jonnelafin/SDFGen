# sdf

SDFGen generates a preferably lower resolution distance field of an image.

This distance field can be upscaled fast with bilinear filters built into gpus nowadays.
Final sharp output of this upscale can be extracted using a threshold filter, with the default parameters in this program, threshold should be set to about 99%.

## Examples
```rust
cargo run --release
```
