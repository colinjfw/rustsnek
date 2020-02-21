use std::convert::TryInto;
use std::u16;

// Int alias so I can change this up.
type Int = u16;
type Pos = (Int, Int);
const INF: u16 = u16::MAX;

struct Snake {
  health: Int,
  body: Vec<(Int, Int)>,
}

struct Board {
  height: Int,
  width: Int,
  me: Int,
  snakes: Vec<Snake>,
  food: Vec<(Int, Int)>,
}

struct FlatGraph<T: Copy> {
  width: Int,
  height: Int,
  list: Vec<T>
}

impl<T: Copy> FlatGraph<T> {
  fn new(w: Int, h: Int, def: T) -> FlatGraph<T> {
    let l: usize = (w * h).try_into().unwrap();
    return FlatGraph{width: w, height: h, list: vec![def; l]};
  }

  fn set(&mut self, x: Int, y: Int, val: T) {
    let i: usize = (self.width * x + y).try_into().unwrap();
    self.list[i] = val;
  }

  fn get(&self, x: Int, y: Int) -> T {
    let i: usize = (self.width * x + y).try_into().unwrap();
    return self.list[i];
  }
}

#[derive(Copy, Clone)]
struct Node {
  start: Int,
  cost: Int,
}

struct Graph {
  width: Int,
  height: Int,
  // Mapping of (prev, start).
  nodes: FlatGraph<Node>,
}

impl Graph {
  fn new(w: Int, h: Int) -> Graph {
    // Initialize to previous 0 and cost infinite.
    let def = Node{start: 0, cost: INF};
    return Graph{
      width: w,
      height: h,
      nodes: FlatGraph::new(w, h, def),
    };
  }
}

impl Board {
  // Draw returns a vector with (prev, cost).
  fn draw(self, initial: Pos) -> Graph {
    let mut graph = Graph::new(self.width, self.height);
    let mut queue = FlatGraph::new(self.width, self.height, false);

    // Set the initial cost to 0.
    graph.nodes.set(initial.0, initial.1, Node{
      start: 0,
      cost: 0,
    });

    while queue.list {
      let cur = unv[0];
      let cur_cost = graph.get(cur.0, cur.1).cost;
    }


    // TODO: Run djikstra and find cost using cost fn for every node in graph.
    // TODO: Need a (neighbours) iterator.
    return graph;
  }

  fn neighbours(self, pos: Pos) -> Neighbours {
    return Neighbours{height: self.height, width: self.width, point: pos, cur: 0}
  }
}

struct Neighbours {
  height: Int,
  width: Int,
  point: Pos,
  cur: u8,
}

impl Iterator for Neighbours {
  type Item = Pos;

  fn next(&mut self) -> Option<Pos> {
    self.cur += 1;
    return match self.cur {
      1 => {
        if self.point.0 + 1 >= self.width {
          return self.next()
        }
        return Option::Some((self.point.0 + 1, self.point.1));
      },
      2 => {
        if self.point.1 + 1 >= self.height {
          return self.next()
        }
        return Option::Some((self.point.0, self.point.1 + 1));
      },
      3 => {
        if self.point.0 <= 0 {
          return self.next()
        }
        return Option::Some((self.point.0 - 1, self.point.1));
      },
      4 => {
        if self.point.1 <= 0 {
          return self.next()
        }
        return Option::Some((self.point.0, self.point.1 - 1));
      },
      _ => Option::None,
    }
  }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbours() {
      let mut n = Neighbours{height: 10, width: 10, point: (0, 0), cur: 0};
      assert_eq!(n.next().unwrap(), (1, 0));
      assert_eq!(n.next().unwrap(), (0, 1));
      assert_eq!(n.next(), Option::None);
    }

    #[test]
    fn test_neighbours_all() {
      let n = Neighbours{height: 10, width: 10, point: (1, 1), cur: 0};
      let mut result = vec![];
      for nb in n {
        result.push(nb)
      }
      assert_eq!(result, [(2, 1), (1, 2), (0, 1), (1, 0)]);
    }

    #[test]
    fn test_flat_graph() {
      let mut fg = FlatGraph::new(10, 10, 0);
      fg.set(0, 0, 1);
      fg.set(9, 9, 2);
      assert_eq!(fg.get(0, 0), 1);
      assert_eq!(fg.get(9, 9), 2);
      assert_eq!(fg.get(5, 5), 0);
    }
}

/**
 * Calculate cost across the board.
 * Look through targets.
 * See cost of target.
 */
fn main() {
    println!("Hello, world!");
}
