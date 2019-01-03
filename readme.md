# Comparison:
A port of [MonoGame's `CreateLookAt` function](https://github.com/MonoGame/MonoGame/blob/develop/MonoGame.Framework/Matrix.cs#L761) to Rust, and a comparison between the two. 
### To run:
- Run project as preconfigured
### To edit rust:
- Edit rust in `./rs-math`
- Compile rust:
  - `cd ./rs-math`
  - `cargo build --release`
- Copy `./rs-math/target/release/rust_math.dll` into your solution, eliminating the previous `dll` in the solution
- Set the `dll` to "Copy Always" under "`rust_math.dll` `>` Properties `>` Copy to Output Directory"
- Run  
Output: An average number of ticks accross 100 times that `CreateLookAt` was called 1'000'000 times with the same input
