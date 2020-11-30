use aoc;
use aoc::GridDrawer;

fn main() {
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
