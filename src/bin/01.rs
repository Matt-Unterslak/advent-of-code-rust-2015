use std::str;
pub struct Floors<'a> {
    floor: i32,
    iter: str::Chars<'a>,
}

impl<'a> Iterator for Floors<'a> {
    type Item = i32;

    fn next(&mut self) -> Option<i32> {
        self.iter.next().map(|ch| {
            match ch {
                '(' => self.floor += 1,
                ')' => self.floor -= 1,
                _ => panic!("invalid character"),
            }
            self.floor
        })
    }
}

impl<'a> Directions<'a> {
    fn new(steps: &str) -> Directions {
        Directions { steps: steps }
    }

    fn floors(&self) -> Floors {
        Floors {
            floor: 0,
            iter: self.steps.chars(),
        }
    }

    fn final_floor(&self) -> Option<i32> {
        self.floors().last()
    }

    fn basement_step(&self) -> Option<usize> {
        self.floors().position(|floor| floor < 0).map(|x| x + 1)
    }
}

pub struct Directions<'a> {
    steps: &'a str,
}

pub fn part_one(input: &str) -> Option<i32> {
    let directions = Directions::new(input);

    return Some(directions.final_floor().unwrap());
}

pub fn part_two(input: &str) -> Option<usize> {
    let directions = Directions::new(input);
    return Some(directions.basement_step().unwrap());
}

advent_of_code::main!(1);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(-1));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", 1));
        assert_eq!(result, Some(5));
    }
}
