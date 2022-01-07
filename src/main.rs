use std::collections::HashSet;

enum Constraint {
    Green(char, usize),
    Yellow(char, usize),
    Grey(char),
}

fn match_constraint(word: &str, constraint: Constraint) -> bool {
    match constraint {
        Constraint::Green(constraint_char, constraint_index) => {
            for (index, letter) in word.chars().enumerate() {
                if index != constraint_index {
                    continue;
                }
                return constraint_char == letter;
            }
            return false;
        }
        Constraint::Yellow(constraint_char, constraint_index) => {
            let mut contains = false;
            for (index, letter) in word.chars().enumerate() {
                if index != constraint_index {
                    if letter == constraint_char {
                        contains = true;
                    }
                }
                if index == constraint_index {
                    if letter == constraint_char {
                        return false;
                    }
                }
            }
            return contains;
        }
        Constraint::Grey(constraint_char) => {
            for char in word.chars() {
                if constraint_char == char {
                    return false;
                }
            }
            return true;
        }
    }
}

fn matches(word: &str, constraints: Vec<Constraint>) -> bool {
    for constraint in constraints {
        if !match_constraint(word, constraint) {
            return false;
        }
    }
    true
}

fn main() {}

#[cfg(test)]
mod tests {
    use crate::{match_constraint, Constraint};

    #[test]
    fn it_works() {
        assert_eq!(match_constraint("tiger", Constraint::Green('c', 0)), false);
        assert_eq!(match_constraint("tiger", Constraint::Green('t', 0)), true);
        assert_eq!(match_constraint("tiger", Constraint::Yellow('t', 0)), false);
        assert_eq!(match_constraint("tiger", Constraint::Yellow('t', 1)), true);
        assert_eq!(match_constraint("tiger", Constraint::Grey('t')), false);
        assert_eq!(match_constraint("tiger", Constraint::Grey('z')), true);
    }
}
