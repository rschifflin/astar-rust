use std::os;
use std::io::buffered::BufferedReader;
use std::io::File;
use std::num;

#[deriving(Eq, Clone)]
enum Symbol {
  Start,
  Finish,
  Open,
  Closed,
  Route
}

struct Node {
  x: u64,
  y: u64,
  cost: u64,
}

fn grid_from_input() -> ~[~[Symbol]] {
  let args = os::args();
  let path = Path::new(args[1]);
  let mut file = BufferedReader::new(File::open(&path));
  let mut grid: ~[~[Symbol]] = ~[];
  for line in file.lines() {
    grid.push(symbolize_line(line));
  }
  grid
}

fn symbolize_line(line: ~str) -> ~[Symbol] {
  let mut symbolized_line: ~[Symbol] = ~[];
  for each_char in line.chars() {
    match each_char {
      's' => symbolized_line.push(Start),
      'f' => symbolized_line.push(Finish),
      'x' => symbolized_line.push(Closed),
      _ => symbolized_line.push(Open)
    }
  }
  symbolized_line
}

trait SymbolIndexable {
  fn at(&self, x:u64, y:u64) -> Symbol;
  fn set(&mut self, x:u64, y:u64, value:Symbol);
  fn find(&self, target:Symbol) -> (u64, u64);
}

impl SymbolIndexable for ~[~[Symbol]] {
  fn at(&self, x:u64, y:u64) -> Symbol {
    self[y][x]
  }

  fn set(&mut self, x:u64, y:u64, value:Symbol) {
    self[y][x] = value;
  }
  
  fn find(&self, target: Symbol) -> (u64, u64) {
    for y in range(0, self.len()) {
      for x in range(0, self[y].len()) {
        if self[y][x] == target {
          return (x as u64, y as u64);
        }
      }
    }
    (0u64, 0u64) 
  }
}

fn solve(grid: ~[~[Symbol]]) -> ~[~[Symbol]] {
  let mut solved_grid = grid.clone();
  let (start_x, start_y):   (u64, u64) = grid.find(Start);
  let (finish_x, finish_y): (u64, u64) = grid.find(Finish);
  let heuristic = score((start_x, start_y), (finish_x, finish_y));
  let node = Node { x: start_x, y: start_y, cost: heuristic };
  
  solved_grid
}

fn score((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
  let x1f = x1 as f64;
  let x2f = x2 as f64;
  let y1f = y1 as f64;
  let y2f = y2 as f64;
  (num::sqrt(num::pow(x1f - x2f, 2.0) + num::pow(y1f - y2f, 2.0)) as u64) + 1
}

fn main() {
  let grid: ~[~[Symbol]] = grid_from_input();
  let solved_grid: ~[~[Symbol]] = solve(grid);
}


//TESTS
#[test]
fn test_symbolize_line_one() {
  assert_eq!(symbolize_line(~"..x.s"), ~[Open, Open, Closed, Open, Start]);
}

#[test]
fn test_symbolize_line_two() {
  assert!(symbolize_line(~".sx.sf..xx.x") == ~[Open, Start, Closed, Open, Start, Finish, Open, Open, Closed, Closed, Open, Closed]);
}

#[test]
fn test_symbolize_line_three() {
  assert!(symbolize_line(~".sx.sf..xx.x") != ~[Start, Open, Closed, Open, Start, Finish, Open, Open, Closed, Closed, Open, Closed]);
}

#[test]
fn test_find_start() {
  let grid: ~[~[Symbol]] = ~[ ~[ Closed, Open ], ~[ Finish, Open ], ~[Closed, Start] ];
  assert!(grid.find(Start) == (1,2) );
}

#[test]
fn test_find_finish() {
  let grid: ~[~[Symbol]] = ~[ ~[ Closed, Open ], ~[ Finish, Open ], ~[Closed, Start] ];
  assert!(grid.find(Finish) == (0,1) );
}

#[test]
fn test_score_0x0_0x0() {
  assert!(score( (0,0), (0,0) ) == 1);
}

#[test]
fn test_score_1x1_3x3() {
  assert!(score( (1,1), (3,3) ) == 3);
}

#[test]
fn test_score_5x9_13x23() {
  assert!(score( (5,9), (13,23) ) == 17);
}

#[test]
#[ignore]
fn with_an_open_grid_of_size_2x2__where_the_topleft_is_start__where_the_bottomright_is_finish__it_makes_a_route_through_the_topright() {
  let grid: ~[~[Symbol]] = ~[ ~[ Start, Open ], ~[ Open, Finish ] ];
  let goal: ~[~[Symbol]] = ~[ ~[ Start, Route], ~[ Open, Finish ] ];
  assert_eq!(solve(grid), goal);
}
