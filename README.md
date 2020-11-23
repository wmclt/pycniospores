# pycniospores

Average FPS with 600 SPORES: 60 FPS.
Suggestion: first do a `cargo build --release` before running it.

## Literature

### About scaling: views
[guide about scaling](https://www.sfml-dev.org/tutorials/2.5/graphics-view.php)

### Storing state
[repo of bincode](https://github.com/servo/bincode)
bincode does use Serde, but is more exactly what I'm looking for.
explanation how to serialize, save to file, use buffered writer, use compression: [blog](https://peteris.rocks/blog/serialize-any-object-to-a-binary-format-in-rust/)

Not sure is (de)serialsation is exactly what I want though: maybe I also need __compression__.
With only serialisation (bincode), the storage would look like this:

```
1  spore: 
u32 + u16 + f32 + f32 + f32 + f32
4 B + 4 B + 4 B + 4 B + 4 B + 4 B = 24 B

2000 spores:
2000 * 24 B = 48 kB

550.000 ticks with 1/100 measurements:
1000 * 48 kB / 100 = 268,8 MB
```

And yet the data should be relatively easy to compress.

Suggestion: [Snap by Gorhill](https://lib.rs/crates/snap)

Btw: maybe only record every 10 / 20 ticks! 

### Cross-compilation
[General](https://rust-lang.github.io/rustup/cross-compilation.html)
[Windows](https://stackoverflow.com/questions/31492799/cross-compile-a-rust-application-from-linux-to-windows)

check this: https://doc.rust-lang.org/nightly/rustc/platform-support.html

Didn't work:, see https://stackoverflow.com/questions/9221236/pkg-config-fails-to-find-package-under-sysroot-directory
For Linux:
```sh
  # installing toolchain for target
  rustup toolchain install stable-x86_64-unknown-linux-gnu
  # adding target
  rustup target add x86_64-unknown-linux-gnu --toolchain stable-x86_64-unknown-linux-gnu 
  ## building for target
  cargo build --release --target x86_64-unknown-linux-gnu  
```

## TODO

* ☑️ try out [data oriented design](http://jamesmcm.github.io/blog/2020/07/25/intro-dod/#en)
* performance
  * ☑️ testing: don't move the spores
  * put force_reach in calibrated_dist
  * put repulsion_dist in calibrated_dist
* use config.rs
* zoom to the centre, not top left
* additional keys
  * F for fullscreen?
  * zoom with touchpad 
  * zoom with mousewheel
  * move with drag
* use config file
* smooth zoom and scroll
  * work with keyup _and_ keydown
  * work with viewstate: STATIC, LEFT, RIGHT, UP, DOWN, ZOOMING_IN, ZOOMING_OUT
* keys to speed up and speed down?
* separation of concerns: simulation, serialization, deserialization, visualization
  * see [bincode](https://github.com/servo/bincode)
  * simulating calculates forces and moves spores
  * positions are buffered as {type, coords, id} structs
  * serialiser serialises these and writes them to file
    * 1 line per tick!
  * deserialiser deserialises
  * visualisation
    * maybe two options:
      * option live
      * option simulate, then watch later
* cross-platform
  * to linux
  * statically to MacOS

## FUTURE / ALTERNATIVELY

* more particle types
* non-linear force equations
* scaling
  * ☑️ use rayon for parallel computing of force vectors
  * acceleration structure
    * divide universe in squares = buckets
    * spores only interact with others in same bucket
* wrap-around
  * either bound the position: self.position = min(self.position, WINDOW_HEIGHT - self.position)
  * or use modulo arithmetics so that spore is always shown in view

## DONE

* ☑️ window
  * ☑️ dark background
  * ☑️ circle
  * ☑️ different colours for circles
* ☑️ universe
  * ☑️ has size
  * ☑️ wraps around itself
* ☑️ "spores"
  * ☑️ have 2-coord, 2-speed, type
  * ☑️ generated in locations
  * ☑️ have repulsion & attraction for different types
  * parameters of repulsion & attraction can easily be tweaked
  * ☑️ movement based on force
  * ☑️ friction: constant deceleration
  * ☑️ spores can work at small scale
* ☑️ show fps and tick
* ☑️ increase fps to 60 by using mesh
* ☑️ memory leak!
* ☑️ forces (not just movements) must be able to wrap around
  * you can see that it doesn't if spores gather at the border of the window
  * solution: map coordinates to closest coordinates, according to x- and y-axis separately
* ☑️ symmetric vs asymmetric forces
  * conclusion: prefer symmetric
* ☑️ spores don't work like in [video](https://www.youtube.com/watch?v=Z_zmZ23grXE)
  * ☑️ symmetric linear function
  * ☑️ variable distances for repulsion and force
  * ☑️ force function only depends on other spore's type, not on own type
*  ☑️ config generation
  *  ☑️ separate module
  *  ☑️ pretty print config
  *  ☑️ printed config must be immediately repluggable instead of randomly generated 
  *  ☑️ from the [rust docs](https://doc.rust-lang.org/std/collections/struct.HashMap.html):
  ```rust
  use std::collections::HashMap;

  let timber_resources: HashMap<&str, i32> = [("Norway", 100), ("Denmark", 50), ("Iceland", 10)].iter().cloned().collect();
  // use the values stored in map
  ```
 * ☑️ zoom
  * ☑️ show zoom
  * ☑️ don't zoom outside of bounds
  * ☑️ zoom CORRECTLY!
* ☑️ move view up, down, left, right
  * ☑️ don't move out of bounds
* ☑️ SPACE to pause
* ☑️ additional fields in Cargo.toml: https://doc.rust-lang.org/cargo/reference/manifest.html
* ☑️ performance
  * ☑️ the loop in calculate_forces() should do as little as possible!!
    * ☑️ remove the triple filter -> __BIG improvement!__
    * ☑️ don't filter out if too far: just force = 0 -> decrease in performance!
  * ☑️ use Vec::with_capacity(usize) instead of Vec::new()
  * ☑️ use crayon for apply_forces()
* ☑️ only show part of universe at a time for a larger universe
  * ☑️ distinction: _view_ is only __part__ of the _universe_
  * ☑️ bound-checking (zoom or move) should still work
* ☑️ Vectors
  * ☑️ use them 
  * ☑️ implement operator Add for Point or Force