# Random Walk Dungeon Generator in Rust

This repository contains a Rust implementation of a dungeon generator using the Random Walk algorithm. The algorithm generates a dungeon-like grid with interconnected paths by performing multiple random walks from different starting points.

## How It Works

The Random Walk algorithm creates paths by randomly moving in one of the four cardinal directions (left, right, up, down) from a starting node. Multiple random walks are performed to create a more complex and interconnected dungeon.

### Data Structures

1. **Enum (Node)**:

   - Represents the type of each cell in the grid.
   - Two possible values: Wall and Floor.

2. **Struct (Grid)**:
   - Represents the grid of nodes.
   - Stores grid dimensions and the nodes vector.

### Random Walk Algorithm

The random walk algorithm works as follows:

1. **Initialization**: Start at a given position `(x, y)` in the grid and set this position to `Floor`.

2. **Random Movement**:

   - For each step, randomly choose a direction from the set {left, right, up, down}.
   - Check if moving in the chosen direction keeps the walker within the grid boundaries.
   - Update the position `(x, y)` based on the chosen direction.
   - Set the new position to `Floor`.

3. **Math Behind the Movement**:
   - Let the current position be `(x, y)`.
   - Randomly generate a number `d` from {0, 1, 2, 3} representing directions {left, right, up, down}.
     - If `d == 0`, move left: `x = x - 1` (`ensure x > 0`).
     - If `d == 1`, move right: `x = x + 1` (ensure `x < width - 1`).
     - If `d == 2`, move up: `y = y - 1` (ensure `y > 0`).
     - If `d == 3`, move down: `y = y + 1` (ensure `y < height - 1`).

### Example

The following example demonstrates how to set up the grid, perform multiple random walks, and print the resulting dungeon:

```rust
fn main() {
    let width = 50;
    let height = 50;
    let num_walks = 10;
    let steps_per_walk = 200;
    let mut grid = Grid::new(width, height);

    grid.generate_dungeon(num_walks, steps_per_walk);

    for y in 0..height {
        for x in 0..width {
            match grid.get_node(x, y) {
                Some(Node::Wall) => print!("#"),
                Some(Node::Floor) => print!("."),
                None => print!(" "),
            }
        }
        println!();
    }
}
```
