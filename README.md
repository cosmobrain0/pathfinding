# A* Pathfinding

This project is split into two files:
* `path.rs`
* `main.rs`

`path.rs` defines a generic A* pathfinding implementation which can work with
nodes which store data and connections which also store data and run functions to find their lengths

`main.rs` uses the Leptos framework to create a CSR webpage for playing around with the pathfinder

Step 1: purpose-built pathfinder: normal pathfinding between points which have
a position, and the distance between two points is how far they are from each
other according to Pythagoras

Step 2: slightly-generic pathfinder: distances work in the same way but h-costs and g-costs aren't part of the nodes given to the pathfinder

Step 3: generic pathfinder: the pathfinder is given a distance function to calculate the length of any connection
