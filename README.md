# irt

Ray-tracer written in [Rust](https://www.rust-lang.org/) for learning and playing around with computer graphics. Based on the [_Ray Tracing in One Weekend_](https://raytracing.github.io/) series of books.

## Example scenes

![Basic scene](https://github.com/user-attachments/assets/ba317fc5-2491-4df4-8e5b-d7981e5c9361)
![Quads with texture rendering and Perlin noise](https://github.com/user-attachments/assets/437ef10d-3e9d-4cb9-8ff4-e5a899849311)
![Cornell box](https://github.com/user-attachments/assets/4b6748a1-3128-4fa4-9759-3b71a2dbb73d)

## Usage

Render a scene with `cargo run --release`. The scene is written to disk as the image file `image.ppm`. 

## Development

### Requirements

- [Rust](https://www.rust-lang.org/)
- Asset files placed in an `assets/` folder. These are not provided.

### Instructions

- Clone the repository
- Render a scene with `cargo run --release`. Scenes can be chosen in [./src/main.rs](./src/main.rs).
