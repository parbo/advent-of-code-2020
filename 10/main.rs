use std::collections::HashMap;
use std::iter::*;

fn make_graph(adapters: &[i64]) -> (i64, i64, aoc::GraphMap<i64, i32, aoc::Directed>) {
    let mut graph = aoc::GraphMap::<i64, i32, aoc::Directed>::new();
    let outlet: i64 = 0;
    let device: i64 = adapters.iter().max().unwrap() + 3;
    let mut a = adapters.to_vec();
    a.push(outlet);
    a.push(device);
    for i in 0..a.len() {
        let gp = graph.add_node(a[i]);
        for j in 0..a.len() {
            if i == j {
                continue;
            }
            let diff = a[j] - a[i];
            if diff >= 1 && diff <= 3 {
                let gnp = graph.add_node(a[j]);
                graph.add_edge(gp, gnp, 1);
            }
        }
    }
    (outlet, device, graph)
}

fn longest_path(
    graph: &aoc::GraphMap<i64, i32, aoc::Directed>,
    u: i64,
    t: i64,
    paths: &mut HashMap<i64, Vec<i64>>,
) -> Vec<i64> {
    if u == t {
        vec![t]
    } else if paths.contains_key(&u) {
        paths.get(&u).unwrap().clone()
    } else {
        let mut max = vec![];
        for c in graph.neighbors(u) {
            let mut p = vec![u];
            p.extend(longest_path(graph, c, t, paths));
            if p.len() > max.len() {
                max = p;
            }
        }
        paths.insert(u, max.clone());
        max
    }
}

fn part1(adapters: &[i64]) -> i64 {
    let (outlet, device, graph) = make_graph(adapters);
    let mut dist = HashMap::new();
    let path = longest_path(&graph, outlet, device, &mut dist);
    let mut num_1 = 0;
    let mut num_3 = 0;
    for i in 0..(path.len() - 1) {
        let diff = path[i + 1] - path[i];
        if diff == 1 {
            num_1 += 1;
        } else if diff == 3 {
            num_3 += 1;
        }
    }
    num_1 * num_3
}

fn find_npaths(
    graph: &aoc::GraphMap<i64, i32, aoc::Directed>,
    u: i64,
    t: i64,
    npaths: &mut HashMap<i64, i64>,
) -> i64 {
    if u == t {
        1
    } else if npaths.contains_key(&u) {
        *npaths.get(&u).unwrap()
    } else {
        let mut sum = 0;
        for c in graph.neighbors(u) {
            sum += find_npaths(graph, c, t, npaths);
        }
        npaths.insert(u, sum);
        sum
    }
}

fn part2(adapters: &[i64]) -> i64 {
    let (outlet, device, graph) = make_graph(adapters);
    let mut npaths = HashMap::new();
    find_npaths(&graph, outlet, device, &mut npaths)
}

fn parse(lines: &[String]) -> Vec<i64> {
    lines.iter().map(|x| x.parse().unwrap()).collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    let result = if part == 1 {
        part1(&parsed)
    } else {
        part2(&parsed)
    };
    println!("{}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        assert_eq!(part2(&vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4,]), 8);
    }
}
