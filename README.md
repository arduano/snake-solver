# Snake solver

A snake game implementation with a few automated solvers (and the solver's visualizations) that attempt to "finish" snake by filling in the whole board without dying.

It relies on some newer rust compiler features, so make sure you run `rustup update stable` if compilation is failing.

To run the showcase script, run:

`bash cargo run --release --example showcase`

## Notable files

The snake game implementation is located in `src/snake/mod.rs`.

The snake solvers are in `src/solvers`, there are 3 main solvers: `basic` which is zigzag, `random_spanning_tree` which is static hamiltonian and `snake_spanning_tree` which is dynamic pathfinding hamiltonian.

## Report

The report for this assignment is located in `report.pdf`
