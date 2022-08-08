Overall Idea/Goal:

- simulation of water flowing over real terrain, projected onto a map

TODOs:

- (/) simple web-app
  - supports es6 modules
  - shows a random ball moving in a canvas
  - a clear update/draw separation
- (/) move update logic into rust
- (/) add very simple rapier 2d sim loop,
  - runs for some number of iterations as part of update loop
  - simulates no items
  - shows nothing
- (/) simple sim of ball falling due to gravity, with 2d position of ball showing
- (/) allow velocity force to be applied to the ball
- (/) some visualisation of applied force
- (/) base velocity force on orientation of device (i.e. based on gravity as felt by device)
- (/) tidy-up implementation so that there is a clearer separation between sensors/rendering and simulation
- (/) get it correctly picking up orientation and using that to control the ball on my ipad
- (/) switch to 3d scene with rolling ball
- (/) encase ball by walls so it can't roll outside
- (/) add multiple balls
- (/) add bumpy floor (based on heightfield)
- (/) derive heightfield from heightfield image
- (/) ensure mappings are correct
- (x) space the random ball placement so that they don't overlap (overlapping balls cause large forces at start, as physics model tries to sort out intersections)
- (x) randomly place balls with a bias towards placement in lower-lying areas
- (x) make it run reasonably with lots of balls
  - (x) rapier tweaks
  - (x) try run some physics on a worker, and display on main
