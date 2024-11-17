# mc-classic-js
 Contains functionality for reading and writing Minecraft Classic JS worlds. 
 
 
 ## What is Minecraft Classic Javascript?
 [Minecraft Classic Javascript](https://classic.minecraft.net/) is an official Mojang port of Minecraft classic that runs inside a web browser. Instead of storing worlds as `.min` or `.dat` files, instead worlds are stored as json objects within a browser's localStorage in the following format:
 
 ```js
savedGame: {"worldSeed":0,"changedBlocks":{},"worldSize":128,"version":1}
 ```
 
Notably, this is very little data, and the keen among you may be able to tell that the tilemap for the world is not stored. Instead the tilemap is regenerated each time the world is loaded, and then an array of changed blocks are placed over. So this library not only reads from and writes into these json objects, but it also generates the tilemap from the seed. This uses rust code that was converted `1:1` from the [deobfuscated javascript world generation code](https://github.com/TheSunCat/Minecraft-Classic-Reversed)

## Usage
 
Add this to your `Cargo.toml`:

```toml
[dependencies]
mc-classic-js = "0.1.2"
```

## Examples

There are a few functions that can read in a savedGame object, depending on whether it is stored inside a db file or just reading in a json string.
To understand the db format, read here.

```rust
use mc-classic-js;

pub fn main() {
    //Default path for Firefox localStorage for classic.minecraft.net, profile and exact path will vary based on user
    let path = String::from(
        "/AppData/Roaming/Mozilla/Firefox/Profiles/########.default-release/storage/default/https+++classic.minecraft.net/ls/data.sqlite"
    );

    //read_saved_game reads in only the savedGame for an sqlite db
    let json_string = read_saved_game(path).unwrap();

    //deserialize_saved_game converts a json string in the savedGame form and turns it into a JSLevel struct
    //Essentially it converts the json object into a rust object
    let level: JSLevel = deserialize_saved_game(json_string);

    //Note the JSLevel struct uses camel case, not snake case. This is intentional so the fields match the original json
    println!("{}",level.worldSeed); 
}
```
