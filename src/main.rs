use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io,
};

fn main() {
    let words = File::open("data/websters.json").unwrap();
    let words: Vec<String> = serde_json::from_reader::<File, Vec<String>>(words).unwrap();

    let mut viable_words: Vec<String> = words.iter().filter(|x| x.len() == 5).cloned().collect();
    println!("viable words: {}", viable_words.len());

    let mut constraints: Vec<Constraint> = vec![];

    loop {
        let best_guess = find_best_guess(&viable_words);

        if best_guess.is_none() {
            println!("Failed to find a word. Sorry!");
            return;
        }

        let guess = best_guess.expect("Checked before");

        println!("You should guess {}", guess);
        println!("Input the result:");

        let mut guess_result = String::new();
        io::stdin()
            .read_line(&mut guess_result)
            .expect("Failed to read line");

        let mut parse_result = parse_constraints(&guess, guess_result.trim());

        while parse_result.is_err() {
            println!("Failed to parse result. Try again:");
            let mut guess_result = String::new();
            io::stdin()
                .read_line(&mut guess_result)
                .expect("Failed to read line");
            parse_result = parse_constraints(&guess, guess_result.trim());
        }

        constraints.extend(parse_result.expect("parse succeeded"));

        viable_words = viable_words
            .iter()
            .filter(|x| matches(x, &constraints))
            .cloned()
            .collect();

        println!("viable words: {}", viable_words.len());
    }
}

enum Constraint {
    Green(char, usize),
    Yellow(char, usize),
    Grey(char),
}

fn match_constraint(word: &str, constraint: &Constraint) -> bool {
    match constraint {
        Constraint::Green(constraint_char, constraint_index) => {
            for (index, letter) in word.chars().enumerate() {
                if index != *constraint_index {
                    continue;
                }
                return *constraint_char == letter;
            }
            false
        }
        Constraint::Yellow(constraint_char, constraint_index) => {
            let mut contains = false;
            for (index, letter) in word.chars().enumerate() {
                if index != *constraint_index && letter == *constraint_char {
                    contains = true;
                }
                if index == *constraint_index && letter == *constraint_char {
                    return false;
                }
            }
            contains
        }
        Constraint::Grey(constraint_char) => {
            for char in word.chars() {
                if *constraint_char == char {
                    return false;
                }
            }
            true
        }
    }
}

fn matches(word: &str, constraints: &[Constraint]) -> bool {
    for constraint in constraints {
        if !match_constraint(word, constraint) {
            return false;
        }
    }
    true
}

fn find_best_guess(words: &[String]) -> Option<String> {
    // build frequency map
    let mut char_frequencies = HashMap::new();
    for word in words {
        for char in word.chars() {
            let frequency = char_frequencies.entry(char).or_insert(0);
            *frequency += 1;
        }
    }

    let mut best = None;

    for word in words {
        match best.clone() {
            Some((_best_word, high_score)) => {
                let mut score = 0;
                for c in word.chars().collect::<HashSet<char>>() {
                    score += *char_frequencies.entry(c).or_insert(0);
                }
                if score > high_score {
                    best = Some((word.clone(), score));
                }
            }
            None => {
                let mut score = 0;
                for c in word.chars() {
                    score += *char_frequencies.entry(c).or_insert(0);
                }
                best = Some((word.clone(), score));
            }
        }
    }

    best.map(|x| x.0)
}

fn parse_constraints(guess: &str, result: &str) -> Result<Vec<Constraint>, ParseError> {
    if result.len() != 5 {
        println!("Result had length {}.", result.len());
        return Err(ParseError {});
    }

    let mut constraints = vec![];

    for (index, (guess_char, result_char)) in guess.chars().zip(result.chars()).enumerate() {
        match result_char {
            'y' => constraints.push(Constraint::Green(guess_char, index)),
            'm' => constraints.push(Constraint::Yellow(guess_char, index)),
            'n' => constraints.push(Constraint::Grey(guess_char)),
            _ => {
                return Err(ParseError {});
            }
        }
    }

    Ok(constraints)
}

#[derive(Debug, Clone)]

struct ParseError {}

#[cfg(test)]
mod tests {
    use crate::{match_constraint, Constraint};

    #[test]
    fn it_works() {
        assert_eq!(match_constraint("tiger", &Constraint::Green('c', 0)), false);
        assert_eq!(match_constraint("tiger", &Constraint::Green('t', 0)), true);
        assert_eq!(
            match_constraint("tiger", &Constraint::Yellow('t', 0)),
            false
        );
        assert_eq!(match_constraint("tiger", &Constraint::Yellow('t', 1)), true);
        assert_eq!(match_constraint("tiger", &Constraint::Grey('t')), false);
        assert_eq!(match_constraint("tiger", &Constraint::Grey('z')), true);
    }
}
