# TriGeRust

Lightweight Rust library for fast geometric calculations and abstract 2D/3D shape modeling.

## Usage
Add to your `Cargo.toml`:
```toml
[dependencies]
tri_ge_rust = "0.1"
```
Then you may do this in your `main.rs`:
```rust
use tri_ge_rust::{two_d, three_d}; // this is so important!

// 2D shapes
let rect = two_d::Rectangle::new(5.0, 3.0).unwrap(); // there are numbers for example, you can use other numbers
println!("Diagonal: {}", rect.diagonal());

// 3D shapes  
let cuboid = three_d::Cuboid::new(4.0, 3.0, 2.0).unwrap(); 
println!("Volume: {}", cuboid.volume());
```
## About feedback
I will be excited, if you send feedback about this library on feedback! Please, report bugs and come up with your greatful ideas!
