#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Cell {
    Empty,
    Tree,
    Burned,
}

struct Forest {
    width: usize,
    height: usize,
    grid: Vec<Cell>,
}

impl Forest {
    fn new_empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            grid: vec![Cell::Empty; width * height],
        }
    }

    #[inline]
    fn idx(&self, r: usize, c: usize) -> usize {
        r * self.width + c
    }

    fn in_bounds(&self, r: isize, c: isize) -> bool {
        r >= 0 && c >= 0 && (r as usize) < self.height && (c as usize) < self.width
    }

    fn at(&self, r: usize, c: usize) -> Cell {
        self.grid[self.idx(r, c)]
    }

    fn set(&mut self, r: usize, c: usize, val: Cell) {
        let i = self.idx(r, c);
        self.grid[i] = val;
    }

    fn print(&self) {
        for r in 0..self.height {
            for c in 0..self.width {
                let ch = match self.at(r, c) {
                    Cell::Empty => ' ',
                    Cell::Tree => 'T',
                    Cell::Burned => 'X',
                };
                print!("{ch}");
            }
            println!();
        }
    }
}

fn main() {
    let forest = Forest::new_empty(10, 5);
    println!("Forest initialized: {}x{}", forest.width, forest.height);
    forest.print();
}
