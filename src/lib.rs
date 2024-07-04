use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Node {
    Wall,
    Floor,
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub nodes: Vec<Node>,
}

impl Grid {
    // Creates a new grid with given width and height, initially filled with walls.
    pub fn new(width: usize, height: usize) -> Self {
        let nodes = vec![Node::Wall; width * height];
        Self {
            width,
            height,
            nodes,
        }
    }

    // Returns the index of the node at position (x, y) in the nodes vector.
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    // Checks if the position (x, y) is within the bounds of the grid.
    fn in_bounds(&self, x: usize, y: usize) -> bool {
        x < self.width && y < self.height
    }

    // Sets the node at position (x, y) to the specified node type.
    fn set_node(&mut self, x: usize, y: usize, node: Node) {
        if self.in_bounds(x, y) {
            let idx = self.idx(x, y);
            self.nodes[idx] = node;
        }
    }

    // Gets the node type at position (x, y) if it's within the bounds of the grid.
    pub fn get_node(&self, x: usize, y: usize) -> Option<Node> {
        if self.in_bounds(x, y) {
            Some(self.nodes[self.idx(x, y)])
        } else {
            None
        }
    }

    // Performs a random walk starting at (start_x, start_y) for a given number of steps.
    fn random_walk(&mut self, start_x: usize, start_y: usize, steps: usize) {
        let mut rng = rand::thread_rng();
        let mut x = start_x;
        let mut y = start_y;

        self.set_node(x, y, Node::Floor);

        for _ in 0..steps {
            let direction = rng.gen_range(0..4);
            match direction {
                0 if x > 0 => x -= 1,               // Move left
                1 if x < self.width - 1 => x += 1,  // Move right
                2 if y > 0 => y -= 1,               // Move up
                3 if y < self.height - 1 => y += 1, // Move down
                _ => continue,
            }
            self.set_node(x, y, Node::Floor);
        }
    }

    // Generates an entire dungeon graph by performing multiple random walks.
    pub fn generate_dungeon(&mut self, num_walks: usize, steps_per_walk: usize) {
        let mut rng = rand::thread_rng();

        for _ in 0..num_walks {
            let start_x = rng.gen_range(1..self.width - 1);
            let start_y = rng.gen_range(1..self.height - 1);
            self.random_walk(start_x, start_y, steps_per_walk);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_initialization() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.nodes.len(), 100);
        for node in grid.nodes {
            assert_eq!(node, Node::Wall);
        }
    }

    #[test]
    fn test_set_and_get_node() {
        let mut grid = Grid::new(10, 10);
        grid.set_node(1, 1, Node::Floor);
        assert_eq!(grid.get_node(1, 1), Some(Node::Floor));
        assert_eq!(grid.get_node(0, 0), Some(Node::Wall));
        assert_eq!(grid.get_node(10, 10), None);
    }

    #[test]
    fn test_random_walk() {
        let mut grid = Grid::new(10, 10);
        grid.random_walk(5, 5, 20);

        let floor_count = grid.nodes.iter().filter(|&&n| n == Node::Floor).count();
        assert!(floor_count > 0, "There should be at least one floor node");
    }

    #[test]
    fn test_generate_dungeon() {
        let mut grid = Grid::new(20, 20);
        grid.generate_dungeon(10, 50);

        let floor_count = grid.nodes.iter().filter(|&&n| n == Node::Floor).count();
        assert!(
            floor_count > 50,
            "There should be a significant number of floor nodes"
        );
    }
}
