# pycniospores

## About scaling: views
https://www.sfml-dev.org/tutorials/2.5/graphics-view.php

## TODO
* window
    * dark background
    * circle
    * different colours for circles
* universe
    * has size
    * wraps around itself
* "spores"
    * have 2-coord, 2-speed, type
    * generated in locations
    * have repulsion & attraction for different types
    * parameters of repulsion & attraction can easily be tweaked
    * movement based on force
    * friction: constant deceleration
* scaling
    * spores can work at small scale
    * acceleration structure
        * divide universe in squares = buckets
        * spores only interact with others in same bucket 
