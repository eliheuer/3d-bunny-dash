# 3D Bunny Dash

A 3D rhythm-jumping game about a pink bunny, inspired by
Geometry Dash — made by a dad and his five-year-old son as
a way to learn math and programming together.

Press **any key** to jump. Spikes kill you. Cubes are
platforms: land on top, but never touch their sides. Reach
the golden gate for fireworks. Nine levels, three bosses:
the Rotten Tomato, the Cursed Thorn, and BAD BAT.

The source code doubles as a beginner lesson book: every
system is commented in plain language explaining the math
that drives it — gravity, sine waves, distance checks,
remainders, pseudo-randomness, and more.

## Play

```sh
cargo run
```

- **Any key** — jump
- **Esc** — back to the title screen
- Title menu: **1** play, **2** settings, **3** level editor

## Level editor

A built-in editor in the spirit of Geometry Dash and Mario
Maker. Arrows move the cursor (hold to zoom), **1–7** place
pieces, **x** deletes, **l** switches levels, **p** playtests,
**s** saves back into `assets/levels.txt` — which is the
game's actual level data, in a format you can also edit by
hand in any text editor.

## Built with

- [Bevy](https://bevy.org/) game engine (Rust)
- All sound effects generated from pure sine-wave math
  (see the git history for the tiny Python scripts)
- The animated background is a little WGSL plasma shader
  (`assets/lava_lamp.wgsl`)

## License

The source code is licensed under the GNU General Public
License v3.0 or later — see [LICENSE](LICENSE).

The fonts in `assets/fonts/` are **not** covered by the
GPL and have their own licenses — see
[assets/fonts/README.md](assets/fonts/README.md).
