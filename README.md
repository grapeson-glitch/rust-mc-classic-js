# mc-classic-js
 Contains functionality for reading and writing Minecraft Classic JS worlds. 
 
 # What is Minecraft Classic Javascript?
 [Minecraft Classic Javascript](https://classic.minecraft.net/) is an official Mojang port of Minecraft classic that runs inside a web browser. Instead of storing worlds as `.min` or `.dat` files, instead worlds are stored as json objects within a browser's localStorage in the following format:
 
 ```json
savedGame: {"worldSeed":0,"changedBlocks":{},"worldSize":128,"version":1}
 ```
 
Notably, this is very little data, and the keen among you may be able to tell that the tilemap for the world is not stored. Instead the tilemap is regenerated each time the world is loaded, and then an array of changed blocks are placed over. So this library not only reads from and writes into these json objects, but it also generates the tilemap from the seed. This uses rust code that was converted `1:1` from the [deobfuscated javascript world generation code](https://github.com/TheSunCat/Minecraft-Classic-Reversed)

# Usage
 
Add this to your `Cargo.toml`:

```toml
[dependencies]
mc-classic-js = "0.1.1"
```
