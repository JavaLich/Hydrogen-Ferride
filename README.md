# Hydrogen-Ferride
2D Newton's Law of Gravity Particle Simulation

## Features
- Universal Law of Gravity, particles attract each other in proportion to mass and inverse to the distance squared
- Inelastic collisions that follow the law of conservation of momentum and make larger particles

## WIP
- Configurable density
- GUI/CLI interface in order to configure simulation constants (eg. number of particles, gravitational constant, size of particles)
- This README

## How to Use
- All constants are located in /src/physics.rs at the top
- COLLISIONS enables inelastic collisions. If it is false, particles will phase right through each other
- PARTICLE_SIZE sets the initial size of the drawn particles
- NUM_PARTICLES sets the initial amount of particles to simulate
- GRAVITATIONAL_CONSTANT sets the gravitational constant or the strength of gravity