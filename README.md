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
mc-classic-js = "0.1.4"
```

## Examples

There are a few functions that can read in a savedGame object, depending on whether it is stored inside a db file or just reading in a json string.
To understand the db format, read here.

```rust
use mc-classic-js;

pub fn main() {
    //Default path for Firefox localStorage for classic.minecraft.net, profile and exact path will vary based on user
    let path = String::from(
        "AppData/Roaming/Mozilla/Firefox/Profiles/########.default-release/storage/default/https+++classic.minecraft.net/ls/data.sqlite"
    );

    //read_saved_game reads in only the savedGame for an sqlite db
    let json_string: String = read_saved_game(path).unwrap();

    //deserialize_saved_game converts a json string in the savedGame form and turns it into a JSLevel struct
    //Essentially it converts the json object into a rust object
    let level: JSLevel = deserialize_saved_game(json_string);

    //Note the JSLevel struct uses camel case, not snake case. This is intentional so the fields match the original json
    println!("{}",level.worldSeed); 
}
```

Similarly, there are multiple functions for writing a level back into the savedGame format. There is functionality for just getting the raw json, for writing it to a db file, and also for writing it to a `localStorage.setItem()` command.

```rust
use mc-classic-js;

pub fn main() {

    let path = (
        "AppData/Roaming/Mozilla/Firefox/Profiles/########.default-release/storage/default/https+++classic.minecraft.net/ls/data.sqlite"
    );

    let seed: i64 = 0; //World seeds are i64
    let mut changed_blocks: HashMap<String, ChangedBlocks> = HashMap::new();
    let world_size: i32 = 128;
    let version: u8 = 1;

    //get_tile_map generates the tile map for the world based on seed and world size
    //This function calls ported classic js world gen code
    let tile_map: Vec<u8> = get_tile_map(seed, world_size);

    let mut level: JSLevel = JSLevel::new(seed, changed_blocks, world_size, version);

    //serialize_saved_game takes a js level and tilemap and writes into a savedGame json string.
    //opt is the third argument, and that is used for optimization based on how much
    //storage space you want the json_string to take up, as it can reach well over a 
    //million characters. 2 is recommended, as it only writes a changedBlock if it is
    //explicitly different from natural generation
    //
    //If opt == 2 the tile must differ from natural generation to write to array
    //If opt == 1 either the tile differs from natural generation or it is already considered a changed block to write to array
    //If opt == 0 tile is written to array
    let json_string: String = serialize_saved_game(level, tile_map, 2);

    //Alternatively, if there is not a js level object, and just a tilemap and a seed,
    //serialize_saved_game_from_seed can be called and a seed and tile_map can be passed
    let json_string1: String = serialize_saved_game_from_seed(seed, tile_map)

    //The savedGame string can be passed to write to a db
    write_saved_game(path, json_string);

    //The savedGame string can be passed to make a localStorage.setItem() command
    //This can be copy/pasted into a browser console. There is also the option
    //to output this command to a txt file, if the path string passed is empty,
    //it will not attempt to write to a file and just return the string
    let set: String = write_saved_game_command("", json_string);

    println!("{}",set);

    
}
```

## Where is the world *actually* stored?

localStorage works differently between different browsers, and currently this library only natively supports Firefox. 

### Firefox

Firefox local storage is stored at

`C:/Users/user/AppData/Roaming/Mozilla/Firefox/Profiles/########.default-release/storage/default/`

Inside this `default` folder, there are folders that correspond to each website that is currently storing data. There are only 2 that are relevant to this code, those being 

```
https+++classic.minecraft.net/ls/data.sqlite
https+++omniarchive.uk/ls/data.sqlite
```

These are the two websites that currently host Minecraft Classic JS. The actual localStorage objects are stored within these `data.sqlite` as key value pairs. Additionally, snappy compression is used on all values stored inside. This means to read a `savedGame`, first the sqlite database has to be opened, then the key `savedGame` has to be found, and then it needs to be decompressed.

### All Browsers

To retreive localStorage manually, this can be done by inspect elementing the browser. From here, either navigate to Local Storage (location varies on browser - just use google at this point) and select the savedGame object manually, or navigate to the console, and run:

```js
localStorage.getItem("savedGame")
```
