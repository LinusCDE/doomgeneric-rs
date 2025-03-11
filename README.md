# doomgeneric-rs

[![cratesio](https://img.shields.io/crates/v/doomgeneric)](https://crates.io/crates/doomgeneric)

This was primarily a test whether a doom port could be integrated into rust without much hassle (to later integrate with another platform).

The awesome doom port [doomgeneric](https://github.com/ozkl/doomgeneric) was used as a basis.

This repo was originally started as [piston-doom](https://github.com/LinusCDE/piston-doom) and later turned into a standalone lib. Therefore this is technically a fork of that repo with the piston-components removed.

![piston-doom](https://transfer.cosmos-ink.net/12oclH/Unbenannt.png)

---

This lib uses [doomgeneric](https://github.com/ozkl/doomgeneric) as a submodule. If you clone this repo locally, make sure to also initialize that submodule. Otherwise building doomgeneric will fail!

The basics are to create a struct that implements the trait `game::DoomGeneric` and pass it to `game::init(...)`. Afterwards you'll need to run `game::tick()` afterwards in a loop.

This library does not support multiple instances. Running `game::init(...)` will pass your struct to the library and keeps it around forever (to use when doomgeneric c functions are called).

A very basic, incomplete usage example:

```rs
struct MyDoomGame {
  // Some fields, like channels to e.g. poll input from or send received framebuffers to
}

impl doomgeneric::game::DoomGeneric for MyDoomGame {
 // Implement all required functions here, they are essentially events from doom
}

fn main() {
  let my_game = MyDoomGame {};
  doomgeneric::game::init(my_game);
  loop {
    doomgeneric::game::tick();
  }
}
```

For more detailed, see the readme of [piston-doom](https://github.com/LinusCDE/piston-doom) and its [main.rs](https://github.com/LinusCDE/piston-doom/blob/main/src/main.rs) for a full, yet short implementation of the above principles.

---

TODO:

- Setting `myargv` and `myargc` to support the `-iwad`-flag: You need to the file named in a default way for now. The console will show you all the names
- Support other resolutions. Not sure if this is possible without forking doomgeneric.
