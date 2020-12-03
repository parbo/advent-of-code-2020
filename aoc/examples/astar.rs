use aoc;
use aoc::GridDrawer;
use petgraph::algo::astar;

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
    if let Some((_cost, path)) = astar(
        &graph,
        a,
        |finish| finish == f, // is finish
        |(_n1, _n2, e)| *e, // true cost
        |n| (f[0] - n[0]).abs() + (f[1] - n[1]).abs(),  // estimated cost: manhattan distance
    ) {
        for point in &path {
            grid[point[1] as usize][point[0] as usize] = '*';
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
}

fn main() {
    astar_char();
}
