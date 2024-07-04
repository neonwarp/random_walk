use random_walk::{Grid, Node};

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
