extern mod extra;

use std::num;
use std::hashmap::HashSet;
use extra::priority_queue;

use node::Node;
use node::UnsafeNodeList;

use grid::Grid;
use grid::grid_from_input;
use grid::Symbol;

mod node;
mod grid;

fn solve(grid: Grid) -> Grid {
  let mut solved_grid = grid.clone();
  let mut open_nodes = priority_queue::PriorityQueue::new();
  let mut working_set = HashSet::new();
  let closed_nodes = UnsafeNodeList { nodes: ~[] };
  let (start_x, start_y):   (u64, u64) = grid.find(Start);
  let (finish_x, finish_y): (u64, u64) = grid.find(Finish);

  let heuristic = score((start_x, start_y), (finish_x, finish_y));
  let node = ~Node{ x: start_x, y: start_y, depth: 0, cost: heuristic, parent: None };
  open_nodes.push(node);
  working_set.insert((start_x, start_y));

  while open_nodes.maybe_top() != None {
    closed_nodes.push(open_nodes.pop());
    let current_node = closed_nodes.last();
    if (current_node.x == finish_x && current_node.y == finish_y) {
      println("Victory!");
      let mut path = ~[];
      let mut parent = current_node.parent;
      path.push((current_node.x, current_node.y));

      while parent != None {
        parent = match parent {
          Some(new_parent) => {
            path.push((new_parent.x, new_parent.y));
            new_parent.parent
          }
          None => None
        }
      }

      for i in path.iter() {
        println!("{:?}", i);
      }

      break;
    }

    else {
      if point_in_bounds(current_node.x - 1, current_node.y, &grid) && !working_set.contains(&(current_node.x - 1, current_node.y)) {
        let new_x = current_node.x - 1;
        let new_y = current_node.y;
        let heuristic = score((new_x, new_y), (finish_x, finish_y));
        let depth = current_node.depth + 1;
        open_nodes.push(~Node{ x: new_x, y: new_y, depth: depth, cost: heuristic + depth, parent: Some(current_node) });
        working_set.insert((new_x, new_y));
      }
      if point_in_bounds(current_node.x + 1, current_node.y, &grid) && !working_set.contains(&(current_node.x + 1, current_node.y)) {
        let new_x = current_node.x + 1;
        let new_y = current_node.y;
        let heuristic = score((new_x, new_y), (finish_x, finish_y));
        let depth = current_node.depth + 1;
        open_nodes.push(~Node{ x: new_x, y: new_y, depth: depth, cost: heuristic + depth, parent: Some(current_node) });
        working_set.insert((new_x, new_y));
      }
      if point_in_bounds(current_node.x, current_node.y - 1, &grid) && !working_set.contains(&(current_node.x, current_node.y - 1)) {
        let new_x = current_node.x;
        let new_y = current_node.y - 1;
        let heuristic = score((new_x, new_y), (finish_x, finish_y));
        let depth = current_node.depth + 1;
        open_nodes.push(~Node{ x: new_x, y: new_y, depth: depth, cost: heuristic + depth, parent: Some(current_node) });
        working_set.insert((new_x, new_y));
      }
      if point_in_bounds(current_node.x, current_node.y + 1, &grid) && !working_set.contains(&(current_node.x, current_node.y + 1)) {
        let new_x = current_node.x;
        let new_y = current_node.y + 1;
        let heuristic = score((new_x, new_y), (finish_x, finish_y));
        let depth = current_node.depth + 1;
        open_nodes.push(~Node{ x: new_x, y: new_y, depth: depth, cost: heuristic + depth, parent: Some(current_node) });
        working_set.insert((new_x, new_y));
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
  let grid: Grid = grid_from_input();
  let solved_grid: Grid = solve(grid);
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
