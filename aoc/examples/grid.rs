use aoc;
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
    g.push(".........".into());
    g.push(".........".into());
    g.push("+.......+".into());
    g
}

fn print_i64() {
    let grid = aoc::parse_grid_to(&make_grid(), |x| match x {
        '.' => 0,
        '#' => 1,
        'A' => 2,
        '+' => 3,
        _ => panic!(),
    });
    let mut gd = aoc::PrintGridDrawer::new(|x| match x {
        0 => '.',
        1 => '#',
        2 => 'A',
        3 => '+',
        _ => panic!(),
    });
    gd.draw(&grid);
}

fn print_char() {
    let grid = aoc::parse_grid(&make_grid());
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&grid);
}

fn save_ppm() {
    let grid = aoc::parse_grid(&make_grid());
    let mut gd = aoc::BitmapGridDrawer::new(
        |x| match x {
            '.' => (255, 255, 255),
            '#' => (0, 0, 0),
            'A' => (0, 255, 0),
            '+' => (0, 0, 255),
            _ => panic!(),
        },
        "ppm/grid/grid",
    );
    gd.draw(&grid);
}

fn main() {
    print_i64();
    println!("");
    print_char();
    save_ppm();
}
