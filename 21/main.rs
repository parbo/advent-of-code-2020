use aoc::Itertools;
use std::collections::{HashMap, HashSet};
use std::iter::*;

type Parsed = Vec<(Vec<String>, Vec<String>)>;

fn find(
    constraints: &HashMap<String, HashSet<String>>,
    sofar: HashMap<String, String>,
    taken: HashSet<String>,
    all: &mut Vec<HashMap<String, String>>,
) {
    if constraints.len() == sofar.len() {
	for a in all.iter() {
	    if *a == sofar {
		return;
	    }
	}
        all.push(sofar.clone());
        return;
    }
    for (allergen, ingredients) in constraints
        .iter()
        .sorted_by(|a, b| a.1.len().cmp(&b.1.len()))
    {
        if sofar.contains_key(allergen) {
            continue;
        }
        for i in ingredients {
            if taken.contains(i) {
                continue;
            }
            let mut new_sofar = sofar.clone();
            new_sofar.insert(allergen.clone(), i.clone());
            let mut new_taken = taken.clone();
            new_taken.insert(i.clone());
            find(constraints, new_sofar, new_taken, all);
        }
    }
}

// Returns (non-allergenic, map of allergen->ingredient)
fn allergenic(input: &Parsed) -> (HashSet<String>, HashMap<String, String>) {
    let mut possible = HashMap::new();
    for (ingredients, allergens) in input {
        for a in allergens {
            let ing: HashSet<String> = ingredients.iter().map(|x| x.clone()).collect();
            let curr = possible.entry(a.clone()).or_insert(ing.clone());
            let updated: HashSet<String> = curr.intersection(&ing).map(|x| x.clone()).collect();
            *curr = updated;
        }
    }
    let mut all = vec![];
    find(&possible, HashMap::new(), HashSet::new(), &mut all);
    let mut allergenic = HashSet::new();
    for res in &all {
        for (_, v) in res {
            allergenic.insert(v);
        }
    }
    let mut na = HashSet::new();
    for (ingredients, _allergens) in input {
        for i in ingredients {
            if !allergenic.contains(i) {
                na.insert(i.clone());
            }
        }
    }
    (na, all[0].clone())
}

fn part1(input: &Parsed) -> usize {
    let (na, _) = allergenic(input);
    let mut num = 0;
    for (ingredients, _allergens) in input {
        for i in ingredients {
	    if na.contains(i) {
		num += 1;
	    }
	}
    }
    num
}

fn part2(input: &Parsed) -> String {
    let (na, allergens) = allergenic(input);
    let mut dangerous = HashSet::new();
    for (ingredients, _) in input {
        for i in ingredients {
	    if !na.contains(i) {
		dangerous.insert(i);
	    }
	}
    }
    // Create the reverse mapping
    let ingredients : HashMap<String, String> = allergens.iter().map(|(k, v)| (v.clone(), k.clone())).collect();
    let mut canonical : Vec<String> = dangerous.into_iter().map(|x| x.clone()).collect();
    canonical.sort_by(|a, b| ingredients.get(a).unwrap().cmp(&ingredients.get(b).unwrap()));
    canonical.iter().join(",")
}

fn parse(lines: &[String]) -> Parsed {
    lines
        .iter()
        .map(|x| {
            let parts = aoc::split(x, |c| c == '(' || c == ')');
            let ingredients = aoc::split_ch(parts[0], ' ')
                .iter()
                .map(|x| x.to_string())
                .collect();
            let allergens = aoc::split_ch(&parts[1][8..], ',')
                .iter()
                .map(|x| x.to_string())
                .collect();
            (ingredients, allergens)
        })
        .collect()
}

fn main() {
    let (part, lines) = aoc::read_lines();
    let parsed = parse(&lines);
    if part == 1 {
        let res = part1(&parsed);
	println!("{}", res)
    } else {
        let res = part2(&parsed);
	println!("{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = vec![
            "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)".to_string(),
            "trh fvjkl sbzzf mxmxvkd (contains dairy)".to_string(),
            "sqjhc fvjkl (contains soy)".to_string(),
            "sqjhc mxmxvkd sbzzf (contains fish)".to_string(),
        ];
        let parsed = parse(&input);
        let expected = vec![
            "kfcds".to_string(),
            "nhms".to_string(),
            "sbzzf".to_string(),
            "trh".to_string(),
        ];
        let na = non_allergenic(&parsed);
        let na_set: HashSet<String> = na.into_iter().collect();
        let expected_set: HashSet<String> = expected.into_iter().collect();
        assert_eq!(na_set, expected_set);
    }
}
