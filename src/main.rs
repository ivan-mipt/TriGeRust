use tri_ge_rust::{three_d, two_d};


fn main() {
    let brick = three_d::Cuboid::new(30.0, 10.0, 5.0).unwrap();
    println!("Just create your projects better! You can use volume of my brick - here it is: {} cm³", brick.volume());
}
