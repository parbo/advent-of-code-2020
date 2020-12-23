use aoc;
use aoc::Grid;
use aoc::GridDrawer;
use aoc::GridTranspose;
use std::collections::HashMap;

fn make_grid() -> Vec<String> {
    let mut g = vec![];
    g.push("+.......+".into());
    g.push(".........".into());
    g.push(".#.......".into());
    g.push(".....AA..".into());
    g.push(".....AABB".into());
    g.push(".####....".into());
    g.push(".#.......".into());
    g.push(".#...####".into());
    g.push(".#...#...".into());
    g.push(".##...#..".into());
    g.push("..####...".into());
    g.push(".....#...".into());
    g.push(".........".into());
    g.push("+.......+".into());
    g
}

fn transform() {
    let orig_grid = aoc::parse_grid(&make_grid());
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    println!("Original:");
    gd.draw(&orig_grid);
    println!();
    println!("Flip horizontal:");
    let mut grid = orig_grid.clone();
    grid.flip_horizontal();
    gd.draw(&grid);
    println!();
    println!("Flip vertical:");
    let mut grid = orig_grid.clone();
    grid.flip_vertical();
    gd.draw(&grid);
    println!();
    println!("Transpose:");
    let mut grid = orig_grid.clone();
    grid.transpose();
    gd.draw(&grid);
    println!();
    println!("Rotate 90 CW / 270 CCW");
    let mut grid = orig_grid.clone();
    grid.rotate_90_cw();
    gd.draw(&grid);
    println!();
    println!("Rotate 180 CW / 180 CCW");
    let mut grid = orig_grid.clone();
    grid.rotate_180_cw();
    gd.draw(&grid);
    println!();
    println!("Rotate 270 CW / 90 CCW");
    let mut grid = orig_grid.clone();
    grid.rotate_270_cw();
    gd.draw(&grid);
    println!();
    println!("Flood fill");
    let mut grid = orig_grid.clone();
    grid.fill([5, 6], 'o');
    gd.draw(&grid);
    println!("Line");
    let mut grid = orig_grid.clone();
    grid.line([7, 2], [1, 11], 'o');
    grid.line([1, 11], [7, 13], 'o');
    gd.draw(&grid);
}

fn transform_sparse() {
    let non_sparse_grid = aoc::parse_grid(&make_grid());
    let mut orig_grid = HashMap::new();
    // Let's make the sparse one not have top left at 0, 0.
    for p in non_sparse_grid.points() {
        if let Some(v) = non_sparse_grid.get_value(p) {
            if v != '.' {
                orig_grid.insert([p[0] - 2, p[1] - 5], v);
            }
        }
    }
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    println!("Original:");
    gd.draw(&orig_grid);
    println!();
    println!("Flip horizontal:");
    let mut grid = orig_grid.clone();
    grid.flip_horizontal();
    gd.draw(&grid);
    println!();
    println!("Flip vertical:");
    let mut grid = orig_grid.clone();
    grid.flip_vertical();
    gd.draw(&grid);
    println!();
    println!("Transpose:");
    let mut grid = orig_grid.clone();
    grid.transpose();
    gd.draw(&grid);
    println!();
    println!("Rotate 90 CW / 270 CCW");
    let mut grid = orig_grid.clone();
    grid.rotate_90_cw();
    gd.draw(&grid);
    println!();
    println!("Rotate 180 CW / 180 CCW");
    let mut grid = orig_grid.clone();
    grid.rotate_180_cw();
    gd.draw(&grid);
    println!();
    println!("Rotate 270 CW / 90 CCW");
    let mut grid = orig_grid.clone();
    grid.rotate_270_cw();
    gd.draw(&grid);
    println!();
    // Skipping flood fill, as it makes less sense in a sparse grid.
    println!("Line (note that it will extend sparse grids)");
    let mut grid = orig_grid.clone();
    grid.line([7, -7], [-6, 3], 'o');
    grid.line([-6, 3], [7, 13], 'o');
    gd.draw(&grid);
}

fn blit() {
    let mut g: Vec<String> = vec![];
    g.push("+........+".into());
    g.push("..........".into());
    g.push("..........".into());
    g.push("..........".into());
    g.push("..........".into());
    g.push("..........".into());
    g.push("..........".into());
    g.push("..........".into());
    g.push("..........".into());
    g.push("+....... +".into());
    let mut g2: Vec<String> = vec![];
    g2.push("...#..#.".into());
    g2.push(".#######".into());
    g2.push("..#..#..".into());
    g2.push("######..".into());
    g2.push("#..#....".into());
    let mut grid = aoc::parse_grid(&g);
    let grid2 = aoc::parse_grid(&g2);
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    grid.blit([1, 2], &grid2);
    gd.draw(&grid);
}

fn transpose_iterator() {
    let orig_grid = aoc::parse_grid(&make_grid());
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    for g in orig_grid.transpositions() {
	gd.draw(&g);
	println!();
    }
}

fn main() {
    println!("== Regular grid ==");
    transform();
    println!();
    println!("== Sparse grid ==");
    transform_sparse();
    println!();
    println!("== Blitting ==");
    blit();
    println!();
    println!("== Iterator over all transpositions ==");
    transpose_iterator();
}
