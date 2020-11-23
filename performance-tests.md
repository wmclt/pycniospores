# release AVG ticks/s (3000 spores) after 120s
34 ticks/s

# release with Data Oriented Design (3000 spores) after 120s
230s : 36 ticks/s

# release with Data Oriented Design => ALSO SPORE_CONFIGS!!! after 120s
6000 (!) spores: 36 fps or AVG ticks/s
5000 spores: 48 AVG ticks/s


all u8
5000: 40.80 fps - 49 MB
all usize
5000: 37.90 fps - 60 MB


# performance with crayon
79  -   74  -   102    -    avg fps
y       y       y           16.3
y       y       n           21.5        <----
y       n       y           5.9
y       n       n           5
n       y       y           14
n       y       n           20.4
n       n       y           5.8
n       n       n           5.5

# tests array vs vec
## array
max 36 fps
min 30 fps
## vec   <------
max 39
min 33