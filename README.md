# mc-classic-js
 Contains functionality for reading and writing Minecraft Classic JS worlds.
 Minecraft classic js stores worlds in json objects, and also does not store
 the tilemap of the world. This means that the tilemap has to be manually 
 generated each time the world is loaded, and also means for the sake of this
 project, the world is essentially generated when read.

# Usage
 
Add this to your `Cargo.toml`:

```toml
[dependencies]
mc-classic-js = "0.1.1"
```
