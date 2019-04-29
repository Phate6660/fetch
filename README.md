# fetch
My info fetch program, written in Rust. Minimalistic (for the most part).

Note: The packages section only works for Arch (or Arch-based) distros as it uses `pacman` and `paclist`.

Note 2: If you want the name of your terminal displayed, ![term](term) must be in the same directory as the fetch binary. Yes, I used a bash script, but I couldn't find a better way to do it. :(

Note 3: I will attempt to add support for the set terminal font, but it looks like a lot of confusing work to get done.

### Requirements
1. `lshw` for the GPU info.
2. `pacman-contrib` for the separate package counts. Will add an option soon to turn those off if you don't want to install it.

**Help**
![Help](Screenshots/help.png?raw=true "Help")

**Default Fetch**
![Default](Screenshots/default.png?raw=true "Default")
