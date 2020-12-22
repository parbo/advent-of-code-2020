use std::collections::{HashSet, VecDeque};
use std::iter::*;

type Parsed = (VecDeque<usize>, VecDeque<usize>);
type Answer = usize;

fn score(win: &VecDeque<usize>) -> usize {
    let mut res = 0;
    for (i, c) in win.iter().enumerate() {
        res += c * (win.len() - i)
    }
    res
}

fn part1(decks: &Parsed) -> Answer {
    let mut a = decks.0.clone();
    let mut b = decks.1.clone();
    loop {
        if a.is_empty() || b.is_empty() {
            break;
        }
        let top_a = a.pop_front().unwrap();
        let top_b = b.pop_front().unwrap();
        if top_a > top_b {
            a.push_back(top_a);
            a.push_back(top_b);
        } else {
            b.push_back(top_b);
            b.push_back(top_a);
        }
    }
    let win = if a.len() > b.len() { a } else { b };
    score(&win)
}

fn recursive_combat(
    a: &mut VecDeque<usize>,
    b: &mut VecDeque<usize>,
    seen: &mut HashSet<(VecDeque<usize>, VecDeque<usize>)>,
) {
    // let mut round = 0;
    loop {
        if a.is_empty() || b.is_empty() {
            break;
        }
        // round += 1;
        let game = seen.len() + 1;
        if game % 10000 == 0 {
            println!("game: {}", game);
        }
        // println!("-- Round {} (Game {}) --", round, game);
        // println!("Player 1's cards: {:?}", a);
        // println!("Player 2's cards: {:?}", b);
        if seen.insert((a.clone(), b.clone())) {
            let top_a = a.pop_front().unwrap();
            let top_b = b.pop_front().unwrap();
            // println!("Player 1's plays: {}", top_a);
            // println!("Player 2's plays: {}", top_b);
            let mut a_wins = top_a > top_b;
            if a.len() >= top_a && b.len() >= top_b {
                let mut aa = a.iter().take(top_a).map(|x| *x).collect();
                let mut bb = b.iter().take(top_b).map(|x| *x).collect();
                // println!("Playing a sub-game to determine the winner...");
		let mut ss = HashSet::new();
                recursive_combat(&mut aa, &mut bb, &mut ss);
                // println!("...anyway, back to game {}", game);
                a_wins = aa.len() > bb.len();
            }
            if a_wins {
                // println!("Player 1 wins round {} of game {}!", round, game);
                a.push_back(top_a);
                a.push_back(top_b);
            } else {
                // println!("Player 2 wins round {} of game {}!", round, game);
                b.push_back(top_b);
                b.push_back(top_a);
            }
        } else {
            // println!("Player 1 wins round {} of game {} due to infinity rule!", round, game);
            // Moving all b's cards to a makes a win
            a.append(b);
        }
    }
}

fn part2(decks: &Parsed) -> Answer {
    let mut a = decks.0.clone();
    let mut b = decks.1.clone();
    let mut seen = HashSet::new();
    recursive_combat(&mut a, &mut b, &mut seen);
    let win = if a.len() > b.len() { a } else { b };
    score(&win)
}

fn parse(lines: &[String]) -> Parsed {
    let players = aoc::split_by_empty_line(lines);
    (
        players[0]
            .iter()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect(),
        players[1]
            .iter()
            .filter_map(|x| x.parse::<usize>().ok())
            .collect(),
    )
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
    fn test_part1() {
        let input = vec![
            "Player 1:".to_string(),
            "9".to_string(),
            "2".to_string(),
            "6".to_string(),
            "3".to_string(),
            "1".to_string(),
            "".to_string(),
            "Player 2:".to_string(),
            "5".to_string(),
            "8".to_string(),
            "4".to_string(),
            "7".to_string(),
            "10".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part1(&parsed), 306);
    }

    #[test]
    fn test_part2() {
        let input = vec![
            "Player 1:".to_string(),
            "9".to_string(),
            "2".to_string(),
            "6".to_string(),
            "3".to_string(),
            "1".to_string(),
            "".to_string(),
            "Player 2:".to_string(),
            "5".to_string(),
            "8".to_string(),
            "4".to_string(),
            "7".to_string(),
            "10".to_string(),
        ];
        let parsed = parse(&input);
        assert_eq!(part2(&parsed), 291);
    }
}
