mod random_level_worker;
mod random;

fn main() {
    println!("Hello, world!");
    let seed: i32 = 0;
    let world_size: i32 = 128;
    random_level_worker::start_generation(world_size, seed);
}
