# midi-reader
a simple app that can search connect to all midi devices and show a graphical representation of what is being played 
made using [slint](https://slint.dev/) and [midir](https://crates.io/crates/midir/0.10.3)

# editing/compiling
all slint files are in the "ui" folder and the entire logic is shared between main.rs(processing incoming input) and input.rs(connection handling) inside of src

# disclaimer
Ai has been used in this project for certain aspects of the logic because I lack the knowledge required for midi devices and their hidden quirks 
most notably the bitwise operations and actually sending the data to slint which required it's own ui thread to keep things smooth and mutex-ing the flipping midi devices together to work hand in hand

...that was rough on my little brain to be honest. but honesty is important
