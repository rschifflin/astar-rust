use std::os;
use std::io::buffered::BufferedReader;
use std::io::File;

#[deriving(Eq, Clone)]
pub enum Symbol {
  Start,
  Finish,
  Open,
  Closed,
  Route
}
pub type Grid = ~[~[Symbol]];

pub fn import_grid() -> Grid {
  let args = os::args();
  let path = Path::new(args[1]);
  let mut file = BufferedReader::new(File::open(&path));
  let mut grid: Grid = ~[];
  for line in file.lines() {
    grid.push(symbolize_line(line));
  }
  grid
}

pub fn export_grid(grid: &Grid) {
  let mut file = File::create(&Path::new("solved_path.txt"));

  for line in grid.iter() {
    file.write_str(stringify_line(line));
  }
}

fn stringify_line(line: &~[Symbol]) -> ~str {
  let mut stringified_line: ~str = ~"";
  for each_sym in line.iter() {
    match *each_sym {
      Open => stringified_line.push_char('.'),
      Closed => stringified_line.push_char('|'),
      Start => stringified_line.push_char('s'),
      Finish => stringified_line.push_char('f'),
      Route => stringified_line.push_char('@')
    }
  }
  stringified_line.push_char('\n');
  stringified_line
}

fn symbolize_line(line: ~str) -> ~[Symbol] {
  let mut symbolized_line: ~[Symbol] = ~[];
  for each_char in line.chars() {
    match each_char {
      's' => symbolized_line.push(Start),
      'f' => symbolized_line.push(Finish),
      '|' => symbolized_line.push(Closed),
      _ => symbolized_line.push(Open)
    }
  }
  symbolized_line
}

pub trait SymbolIndexable {
  fn at(&self, x:u64, y:u64) -> Symbol;
  fn set(&mut self, x:u64, y:u64, value:Symbol);
  fn find(&self, target:Symbol) -> (u64, u64);
}

impl SymbolIndexable for Grid {
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

