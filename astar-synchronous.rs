extern mod extra;

use std::num;
use std::hashmap::HashMap;
use extra::priority_queue;

use node::Node;

use grid::{Start, Finish, Open, Closed, Route};
use grid::Grid;
use grid::{import_grid, export_grid};
use grid::SymbolIndexable;

mod node;
mod grid;

fn solve(grid: Grid) -> Grid {
  let mut solved_grid = grid.clone();
  let mut open_nodes = priority_queue::PriorityQueue::new();
  let mut working_set = HashMap::new();
  let (start_x, start_y):   (u64, u64) = grid.find(Start);
  let (finish_x, finish_y): (u64, u64) = grid.find(Finish);
  let heuristic = score((start_x, start_y), (finish_x, finish_y));
  let node = ~Node{ x: start_x, y: start_y, depth: 0, cost: heuristic, parent_x: start_x, parent_y: start_y };
  working_set.insert((node.x, node.y), (node.parent_x, node.parent_y));
  open_nodes.push(node);

  while open_nodes.maybe_top() != None {
    let current_node = open_nodes.pop();

    if (current_node.x == finish_x && current_node.y == finish_y) {
      let mut current = (current_node.parent_x, current_node.parent_y);
      let mut parent = *(working_set.get(&current));
      while current != parent {
        let (x,y) = current;
        solved_grid.set(x, y, Route);
        current = *(working_set.get(&current));
        parent = *(working_set.get(&current));
      }
      break;
    }

    else {
      let offset_list = [(-1, 0), (1, 0), (0, -1), (0, 1)];
      for offset in offset_list.iter() {
        let (offset_x, offset_y) = *offset;
        let parent_x = current_node.x;
        let parent_y = current_node.y;
        let new_x = parent_x - offset_x;
        let new_y = parent_y - offset_y;

        if point_in_bounds(new_x, new_y, &grid) && !working_set.contains_key(&(new_x, new_y)) {
          let heuristic = score((new_x, new_y), (finish_x, finish_y));
          let depth = current_node.depth + 1;
          working_set.insert((new_x, new_y), (parent_x, parent_y));
          open_nodes.push(~Node{ x: new_x, y: new_y, depth: depth, cost: heuristic + depth, parent_x: parent_x, parent_y: parent_y });
        }
      }
    }
  }

  solved_grid
}

fn score((x1, y1): (u64, u64), (x2, y2): (u64, u64)) -> u64 {
  let delta_x = x1 as f64 - x2 as f64;
  let delta_y = y1 as f64 - y2 as f64;
  let euclidean_distance = (num::sqrt(num::pow(delta_x, 2.0) + num::pow(delta_y, 2.0)) as u64);
  euclidean_distance + 1
}

fn point_in_bounds(x: u64, y: u64, grid: &Grid) -> bool {
  if y >= grid.len() as u64 || x >= grid[0].len() as u64 {
    return false;
  }

  grid.at(x,y) != Closed
}

fn main() {
  let grid: Grid = import_grid();
  let solved_grid: Grid = solve(grid);
  export_grid(&solved_grid);
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
  let grid: Grid = ~[ ~[ Start, Open ], ~[ Open, Finish ] ];
  let goal: Grid = ~[ ~[ Start, Route], ~[ Open, Finish ] ];
  assert_eq!(solve(grid), goal);
}
