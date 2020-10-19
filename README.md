# pycniospores

## About scaling: views
[guide about scaling](https://www.sfml-dev.org/tutorials/2.5/graphics-view.php)

## About (de)serialisation: servo/bincode
[repo of bincode](https://github.com/servo/bincode)

bincode does use Serde, but is more exactly what I'm looking for.

## TODO
* show tick: see piston examples > hello_world
* spores don't work like in [video](https://www.youtube.com/watch?v=Z_zmZ23grXE)
  * symmetric linear function
  * variable distances for repulsion and force
  * force function only depends on other spore's type, not on own type
* scaling
  * use rayon for parallel computing of force vectors
  * acceleration structure
    * divide universe in squares = buckets
    * spores only interact with others in same bucket
* separation of concerns: simulation, serialization, deserialization, visualization
  * simulating calculates forces and moves spores
  * positions are buffered as {type, coords, id} structs
  * serialiser serialises these and writes them to file
    * 1 line per tick!
  * deserialiser deserialises
  * visualisation
* maybe two options:
  * option live
  * option simulate, then watch later

## FUTURE

* more particle types
* non-linear force equations
* symmetric vs asymmetric forces

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
