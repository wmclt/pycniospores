# pycniospores

## About scaling: views
https://www.sfml-dev.org/tutorials/2.5/graphics-view.php

## TODO
* spores don't work like in video https://www.youtube.com/watch?v=Z_zmZ23grXE
    * symmetric linear function 
    * variable distances for repulsion and force
    * force function only depends on other spore's type, not on own type
* scaling
    * acceleration structure
        * divide universe in squares = buckets
        * spores only interact with others in same bucket 

## FUTURE
* more particles
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