use std::iter::*;

fn part1(rules: &[(String, Vec<(usize, String)>)]) -> i64 {
    let mut graph = aoc::GraphMap::<&String, i32, aoc::Directed>::new();
    for (node, neighbors) in rules {
        let gp = graph.add_node(node);
        for (_c, n) in neighbors {
            let gnp = graph.add_node(n);
            graph.add_edge(gp, gnp, 1);
        }
    }
    let mut with_path = 0;
    for (node, _) in rules {
        if node != "shiny gold"
            && aoc::algo::has_path_connecting(&graph, node, &"shiny gold".to_string(), None)
        {
            with_path += 1;
        }
    }
    with_path
}

fn sum_bags(bag: &str, rules: &[(String, Vec<(usize, String)>)]) -> usize {
    for (node, neighbors) in rules {
        if node == bag {
            let mut tot = 1;
            for (c, n) in neighbors {
                tot += c * sum_bags(n, rules);
            }
            return tot;
        }
    }
    0
}

fn part2(rules: &[(String, Vec<(usize, String)>)]) -> i64 {
    // off by one for some reason
    sum_bags("shiny gold", rules) as i64 - 1
}

fn parse(lines: &[String]) -> Vec<(String, Vec<(usize, String)>)> {
    lines
        .iter()
        .map(|x| {
            let parts = aoc::split_str(x, "bags contain");
            (
                parts[0].to_string(),
                aoc::split_ch(&parts[1], ',')
                    .iter()
                    .map(|x| aoc::split_ch(x, ' '))
                    .map(|x| {
                        if x[0] == "no" && x[1] == "other" {
                            (0, x[0..2].join(" "))
                        } else {
                            (
                                x[0].parse::<usize>().unwrap(),
                                x[1..3].join(" ").to_string(),
                            )
                        }
                    })
                    .collect(),
            )
        })
        .collect()
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
    fn test_sum_bag() {
        let lines = vec![
            "light; red bags contain 1 bright white bag, 2 muted yellow bags.".to_string(),
            "dark orange bags contain 3 bright white bags, 4 muted yellow bags.".to_string(),
            "bright white bags contain 1 shiny gold bag.".to_string(),
            "muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.".to_string(),
            "shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.".to_string(),
            "dark olive bags contain 3 faded blue bags, 4 dotted black bags.".to_string(),
            "vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.".to_string(),
            "faded blue bags contain no other bags.".to_string(),
            "dotted black bags contain no other bags.".to_string(),
        ];
        let rules = parse(&lines);
        assert_eq!(part2(&rules), 32);
    }

    #[test]
    fn test_sum_bag2() {
        let lines = vec![
            "shiny gold bags contain 2 dark red bags.".to_string(),
            "dark red bags contain 2 dark orange bags.".to_string(),
            "dark orange bags contain 2 dark yellow bags.".to_string(),
            "dark yellow bags contain 2 dark green bags.".to_string(),
            "dark green bags contain 2 dark blue bags.".to_string(),
            "dark blue bags contain 2 dark violet bags.".to_string(),
            "dark violet bags contain no other bags.".to_string(),
        ];
        let rules = parse(&lines);
        assert_eq!(part2(&rules), 126);
    }
}
