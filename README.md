# amethyst_2d_platformer

2d platformer made in [Amethyst](https://github.com/amethyst/amethyst), the [Rust](https://github.com/rust-lang/rust) game engine. The goal of this project is to develop a usable starter project or template for other Amethyst users.  

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