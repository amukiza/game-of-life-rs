pub mod game {
    #[derive(PartialEq, Debug)]
    pub struct Cell {
        pub x: i32,
        pub y: i32
    }

    pub struct World {
        pub cells: Vec<Cell>
    }

    impl Cell {
       pub fn new(x: i32, y: i32) -> Cell {
            Cell {x, y}
       }

       pub fn will_survive_in(&self, cells: &Vec<Cell>) -> bool {
           let count = cells
               .iter()
               .filter( |c| self.is_neighbour_to(c) )
               .count();

           if self.is_alive(cells) {
             return count > 1 && count < 4
           }
           count == 3
       } 

       fn is_neighbour_to(&self, other: &Cell) -> bool {
          (self.x - other.x).abs() <= 1 && (self.y - other.y).abs() <= 1 && self != other
       }

       fn is_alive(&self, cells: &Vec<Cell>) -> bool {
           cells.iter().any( |c| self == c )
       }

    }

    impl World {

        fn new(cells: Vec<Cell>) -> World {
            World { cells }
        }

       pub fn living_cells(&self) -> Vec<Cell> {
           self.cells
       }

       pub fn spawn(&self) -> World {
           let next_gen = self.cells
               .iter()
               .filter(|c| c.will_survive_in(&self.cells))
               .collect();

           self.seed(next_gen)
       }

        fn seed(self, cells: Vec<Cell>) -> World {
            World { cells }
        }
    }
}

#[cfg(test)]
mod game_tests {
    #[cfg(test)]
    mod cell_tests {
        use game::{Cell};

        #[test]
        fn it_returns_false_cell_has_no_neighbours() {
            let n = Cell::new(0, 0);
            let cells = Vec::new();

            assert!(!n.will_survive_in(&cells))
        }

        #[test]
        fn it_returns_false_when_cell_has_two_neighbours() {
            let n = Cell::new(0, 0);
            let cells = vec![Cell::new(0, 0), Cell::new(0, 1), Cell::new(1, 0)];

            assert!(n.will_survive_in(&cells))
        }

        #[test]
        fn it_returns_false_if_cell_has_more_than_3_neighbours() {
            let n = Cell::new(1, 1);
            let cells = vec![Cell::new(0, 0), Cell::new(0, 1), Cell::new(1, 0), Cell::new(2,1), Cell::new(1, 1), Cell::new(2, 0)];

            assert!(!n.will_survive_in(&cells))
        }

        #[test]
        fn it_returns_true_if_cell_is_dead_and_has_three_neighbours() {
            let n = Cell::new(1, 1);
            let cells = vec![Cell::new(0, 0), Cell::new(0, 1), Cell::new(1, 0)];

            assert!(n.will_survive_in(&cells))
        }
    }

    #[cfg(test)]
    mod world_tests {
        use game::{Cell, World};
        #[test]
        fn test_spawn_new_word() {
            let cells = vec![
                Cell::new(0, 0),
                Cell::new(0, 1),
                Cell::new(1, 0),
                Cell::new(1, 1),
                Cell::new(3, 3)
            ];
            let w = World { cells };

            let word = w.spawn();
            let expected = vec![Cell::new(0, 0), Cell::new(0, 1), Cell::new(1, 0), Cell::new(1, 1)];

            assert_eq!(expected, word.living_cells());
        }
    }
}
