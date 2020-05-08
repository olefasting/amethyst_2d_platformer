# amethyst-2d-platformer

2d platformer made in [Amethyst](https://amethyst.rs/), the [Rust](https://www.rust-lang.org/) game engine. The goal of this project is to develop a usable starter project or template for other Amethyst users.  

Uses ncollide2d for collisions. Everything else is made from the ground up. There are a lot of bugs in the code yet, as it is still in an early stage, so it is obviously not meant for any real use, except for learning purposes, as of now.  

Pull requests and issues are very welcome; the latter both for actual bugs, as well as any suggestions regarding coding practice, etc. I am rather new to Amethyst and I am not a pro Rust dev, as of yet (though I have extensive programming experience, both as a hobby and professionally), so there might be some examples of bad practice in the code. If found, I would really appreciate an issue on the matter. Thanks, in advance!  

Have fun!  

## How to run

To run the game, run the following command, which defaults to the `vulkan` graphics backend:

```bash
cargo run
```

Windows and Linux users may explicitly choose `"vulkan"` with the following command:

```bash
cargo run --no-default-features --features "vulkan"
```

Mac OS X users may explicitly choose `"metal"` with the following command:

```bash
cargo run --no-default-features --features "metal"
```

Credits for the [player sprite](https://github.com/olefasting/capstone/blob/master/assets/textures/player.png) goes to [Clint Bellanger](https://opengameart.org/content/platformer-animations) and it is licensed under [CC-BY 3.0](https://creativecommons.org/licenses/by/3.0/).  

Copyright 2020 Ole A. Sjo Fasting  

License: MIT