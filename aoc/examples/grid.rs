use aoc;
use aoc::GridDrawer;

fn print_i64() {
    let mut g = vec![];
    g.push(vec![3, 0, 0, 0, 0, 0, 0, 0, 3]);
    g.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    g.push(vec![0, 1, 0, 0, 0, 0, 0, 0, 0]);
    g.push(vec![0, 0, 0, 0, 0, 2, 2, 0, 0]);
    g.push(vec![0, 0, 0, 0, 0, 2, 2, 0, 0]);
    g.push(vec![0, 1, 1, 1, 1, 0, 0, 0, 0]);
    g.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    g.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    g.push(vec![0, 0, 0, 0, 0, 0, 0, 0, 0]);
    g.push(vec![3, 0, 0, 0, 0, 0, 0, 0, 3]);
    let mut gd = aoc::PrintGridDrawer::new(|x| match x {
        0 => '.',
        1 => '#',
        2 => 'A',
        3 => '+',
        _ => panic!(),
    });
    gd.draw(&g);
}

fn print_char() {
    let mut g = vec![];
    g.push("+.......+".chars().collect());
    g.push(".........".chars().collect());
    g.push(".#.......".chars().collect());
    g.push(".....AA..".chars().collect());
    g.push(".....AA..".chars().collect());
    g.push(".####....".chars().collect());
    g.push(".........".chars().collect());
    g.push(".........".chars().collect());
    g.push(".........".chars().collect());
    g.push("+.......+".chars().collect());
    let mut gd = aoc::PrintGridDrawer::new(|x| x);
    gd.draw(&g);
}

fn main() {
    print_i64();
    println!("");
    print_char();
}
