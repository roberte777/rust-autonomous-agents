# Rust Boid Simulation

_This is my first Rust project, so if your eyes burn looking at the code, my apologies_

This project aims to simulate a flocking mechanism in Rust. I used nannou for visual representation and vectors.

The three components that make up the flocking mechanism are:

- alignment
- cohesion
- separation

In this project I also investigate some simpler mechanisms, such as fleeing, pursuing, and seeking.

## Execution

To run the flocking simulation, run the following command:

```
cargo run --bin flock
```

All other experiments can be executed by replacing flock for the respective file name in the bin folder.

## Example

[Testing](./media_examples/flocking.png)
