mod random_level_worker;
mod random;
use std::collections::HashMap;

fn main() {
    println!("Starting World Generation");
    let seed: i32 = 0;
    let world_size: i32 = 128;
    let level: HashMap<usize, u8> = random_level_worker::start_generation(world_size, seed);

    //Debug code for listing all blocks in the level
    for i in 0..world_size * world_size * 64 {
        let tile: u8 = level.get(&(i as usize)).copied().unwrap_or(255);
        if tile != 255 {
            //println!("{}:{}",i,tile);
        } else {
            println!("HEY IDIOT - YOU HAVE AN ERROR AT {}!",i);
        }
        
    }
}
