use aoc;
use aoc::Grid;
use aoc::GridDrawer;

fn make_grid() -> Vec<String> {
    let mut g = vec![];
    g.push("+.......+".into());
    g.push(".........".into());
    g.push(".#.......".into());
    g.push(".....AA..".into());
    g.push(".....AA..".into());
    g.push(".####....".into());
    g.push(".........".into());
    g.push(".....####".into());
    g.push(".....#...".into());
    g.push("..#......".into());
    g.push("..####...".into());
    g.push(".....#...".into());
    g.push(".........".into());
    g.push("+.......+".into());
    g
}

fn transform() {
    let mut grid = aoc::parse_grid(&make_grid());
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
    println!();
    grid.flip_horizontal();
    gd.draw(&grid);
    println!();
    grid.flip_vertical();
    gd.draw(&grid);
}

fn main() {
    transform();
}
