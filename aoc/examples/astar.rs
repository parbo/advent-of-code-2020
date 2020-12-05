use aoc;
use aoc::Grid;
use aoc::GridDrawer;

fn make_grid() -> Vec<String> {
    let mut g = vec![];
    g.push("#########".into());
    g.push("#.......#".into());
    g.push("#####...#".into());
    g.push("#.......#".into());
    g.push("#..######".into());
    g.push("##......#".into());
    g.push("#####...#".into());
    g.push("#...#...#".into());
    g.push("#.......#".into());
    g.push("#########".into());
    g
}

fn astar_char() {
    let mut grid = aoc::parse_grid(&make_grid());
    let graph = aoc::grid_to_graph(&grid, |_p, c| *c == '.', |_p1, _c1, _p2, _c2| Some(1), 4);
    let a: aoc::Point = [1, 1];
    let f: aoc::Point = [1, 7];
    if let Some((_cost, path)) = aoc::astar(&graph, a, f) {
        for point in &path {
            grid.set_value(*point, '*');
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
}

fn astar_char_sparse() {
    let mut grid =
        aoc::parse_grid_to_sparse(&make_grid(), |c| if c == '.' { Some(c) } else { None });
    let graph = aoc::grid_to_graph(&grid, |_p, c| *c == '.', |_p1, _c1, _p2, _c2| Some(1), 4);
    let a: aoc::Point = [1, 1];
    let f: aoc::Point = [1, 7];
    if let Some((_cost, path)) = aoc::astar(&graph, a, f) {
        for point in &path {
            grid.set_value(*point, '*');
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
}

fn main() {
    astar_char();
    println!();
    astar_char_sparse();
}
