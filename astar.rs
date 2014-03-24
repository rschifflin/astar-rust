use std::os;
use std::io::buffered::BufferedReader;
use std::io::File;

#[deriving(Eq)]
enum Symbol {
  Start,
  Finish,
  Open,
  Closed
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
}

impl SymbolIndexable for ~[~[Symbol]] {
  fn at(&self, x:u64, y:u64) -> Symbol {
    self[y][x]
  }
}

fn main() {
  let grid: ~[~[Symbol]] = grid_from_input();
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
