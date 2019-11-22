use std::convert::TryInto;
use std::i16;

struct Snake {
  health: i32,
  body: Vec<(i32, i32)>,
}

struct Board {
  height: i32,
  width: i32,
  me: i32,
  snakes: Vec<Snake>,
  food: Vec<(i32, i32)>,
}

impl Board {
  // Draw returns a vector with (prev, cost).
  fn draw(self) -> Vec<(i16, i16)> { // TODO: Add a cost function here.
    let size = ((self.height * self.width)).try_into().unwrap();
    let vec = vec![(0, i16::MAX); size];
    // TODO: Run djikstra and find cost using cost fn for every node in graph.
    // TODO: Need a (neighbours) iterator.
    return vec;
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        // assert_eq!(add(2, 2), 4);
    }
}

fn main() {
    println!("Hello, world!");
}
