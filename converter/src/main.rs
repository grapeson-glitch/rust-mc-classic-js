mod random_level_worker;
mod random;
use std::collections::HashMap;
use std::fs;

fn main() {
    println!("Starting World Generation");
    let seed: i32 = 47;
    let world_size: i32 = 128;
    let level: HashMap<usize, u8> = random_level_worker::start_generation(world_size, seed);

    //Debug code for finding missing blocks in the level
    for i in 0..world_size * world_size * 64 {
        let tile: u8 = level.get(&(i as usize)).copied().unwrap_or(255);
        if tile != 255 {
            //println!("{}:{}",i,tile);
        } else {
            println!("HEY IDIOT - YOU HAVE AN ERROR AT {}!",i);
        }
        
    }

    format_saved_game(seed, level, world_size, 1);
}

pub fn format_saved_game (seed: i32, level: HashMap<usize, u8>, world_size: i32, version: u8) {

    let loc_open: String = String::from(r#"localStorage.setItem("SavedGame", `"#);
    let loc_close: String = String::from(r#"`)"#);

    //Assigning x, y, and z of world
    let x: i32 = world_size;
    let y: i32 = 64;
    let z: i32 = world_size;

    let mut output: String = String::from("{"); //Opening json object

    output += &format!(r#""worldSeed":{},"#,seed.to_string()); //Adding seed key value pair

    //Adding changed blocks key value pair
    output += r#""changedBlocks":"#; //Adding blocks key
    output += "{"; //Opening block values object

    //Variables for the tiles and a value
    let mut t: u8 = 0;
    let mut t2: u8 = 0;
    let mut a: u8 = 0; //a = 0 if changed block matches generation, a = 1 if changed block does not match generation

    //Iterating through all blocks
    for i in 0..x {
        for j in 0..y {
            for k in 0..z {

                //Setting tile for changed block
                t = level.get(&((i + j + k) as usize)).copied().unwrap_or(255);
                t2 = t; //Unimplemented code for checking a saved game tilemap against the generation to determine whether 'a' should be 1 or 0
                if t == t2 { a = 0 } else { a = 1 }

                //Creating key for changed block
                output += &format!(r#""p{}_{}_{}":"#,i,j,k);

                //Creating value for changed block
                output += "{";
                output += &format!(r#""a":{},"bt":{}"#,a,t);
                output += "},";

            }
        }
    }

    output.pop(); //Removing extra comma
    output += "},"; //Closing Changed Blocks object

    output += &format!{r#""worldSize":{},"#,world_size}; //Adding world size key value pair
    output += &format!{r#""version":{}"#,version}; //Adding version key value pair

    output += "}"; //Closing json object
    fs::write("../localStorage.txt", &format!("{}{}{}",loc_open,output,loc_close)).expect("The dude writing this code is an idiot...");

    //println!("{}",output);
}
