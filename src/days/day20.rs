use std::{cell::Cell, error::Error};

// Initial arrangement:
// 0-7
// [1, 2, -3, 3, -2, 0, 4]
//
// 1 moves between 2 and -3:
// 1-2  0-1  2-7
// [2], [1], [-3, 3, -2, 0, 4]
//
// 2 moves between -3 and 3:
// 0    2     1    3    4-7
// [1], [-3], [2], [3], [-2, 0, 4]
//
// -3 moves between -2 and 0:
// 0    1    3    4     2     5-7
// [1], [2], [3], [-2], [-3], [0, 4]
//
// 3 moves between 0 and 4:
// 0    1    4     2     5    4    6
// [1], [2], [-2], [-3], [0], [3], [4]

type ID = usize;

#[derive(Eq)]
struct Segment {
    id: usize,
    datum: i32,
    previous: Cell<ID>,
    next: Cell<ID>,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.datum == other.datum
    }
}

struct Mixer {
    input: Vec<i32>,
    current: usize,
    list: Vec<Segment>,
    head: ID,
}

impl Mixer {
    fn new(input: Vec<i32>) -> Self {
        let list = Self::build_segments(&input);
        Self {
            input,
            current: 0,
            list,
            head: 0,
        }
    }

    fn build_segments(input: &Vec<i32>) -> Vec<Segment> {
        let segments: Vec<Segment> = input
            .iter()
            .enumerate()
            .map(|(i, d)| Segment {
                id: i,
                datum: *d,
                next: Cell::new(if i == input.len() - 1 { 0 } else { i + 1 }),
                previous: Cell::new(if i == 0 { input.len() - 1 } else { i - 1 }),
            })
            .collect();

        segments
    }

    fn mix_all(mut self) -> Vec<i32> {
        while let Some(_) = self.mix_one() {}
        self.result()
    }

    fn mix_one(&mut self) -> Option<()> {
        if self.current >= self.input.len() {
            return None;
        }

        let current = &self.list[self.current];
        if current.datum.is_positive() {
            for _ in 0..current.datum {
                let swapped = current.move_right(self);
                if self.head == current.id {
                    self.head = swapped.id;
                }
            }
        } else if current.datum.is_negative() {
            for _ in 0..current.datum.abs() {
                let swapped = current.move_left(self);
                if self.head == current.id {
                    self.head = swapped.id;
                }
            }
        }

        self.current += 1;
        debug_assert!(self.check_cycles().is_none());
        Some(())
    }

    fn get(&self, id: ID) -> &Segment {
        &self.list[id]
    }

    fn check_cycles(&self) -> Option<ID> {
        let mut t = self.head;
        let mut h = self.head;
        loop {
            t = self.get(t).next.get();
            h = self.get(h).next.get();
            h = self.get(h).next.get();
            if t == self.head {
                return None;
            } else if t == h {
                return Some(t);
            }
        }
    }

    fn result(&self) -> Vec<i32> {
        let mut current = self.head;
        let mut data = Vec::with_capacity(self.input.len());
        loop {
            let segment = self.get(current);
            data.push(segment.datum);
            current = segment.next.get();
            if current == self.head {
                break;
            }
        }

        data
    }
}

impl Segment {
    fn move_left<'a>(&self, mixer: &'a Mixer) -> &'a Segment {
        let p1 = mixer.get(self.previous.get());
        let p2 = mixer.get(p1.previous.get());
        let n = mixer.get(self.next.get());
        // From this:
        // p2 <-> p1 <-> self <-> n
        // To this:
        // p2 <-> self <-> p1 <-> n
        p2.set_next(self);
        self.set_next(p1);
        p1.set_next(n);
        p1
    }

    fn move_right<'a>(&self, mixer: &'a Mixer) -> &'a Segment {
        // From this:
        // p -> self -> n1 -> n2
        // To this:
        // p -> n1 -> self -> n2

        let p = mixer.get(self.previous.get());
        let n1 = mixer.get(self.next.get());
        let n2 = mixer.get(n1.next.get());
        p.set_next(n1);
        n1.set_next(self);
        self.set_next(n2);
        n1
    }

    fn set_next(&self, next: &Segment) {
        self.next.set(next.id);
        next.previous.set(self.id);
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let input: Vec<i32> = input.lines().map(|s| s.parse()).collect::<Result<_, _>>()?;
    let result = Mixer::new(input).mix_all();
    let i = result.iter().position(|i| *i == 0).unwrap();
    let k1 = result[(i + 1000) % result.len()];
    let k2 = result[(i + 2000) % result.len()];
    let k3 = result[(i + 3000) % result.len()];
    println!("i={i} k1={k1}, k2={k2}, k3={k3}");
    Ok((k1 + k2 + k3).to_string())
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day20test.txt");

    #[test]
    fn test_mix() {
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        let mut mixer = Mixer::new(input);
        mixer.mix_one();
        assert_eq!(mixer.result(), vec![2, 1, -3, 3, -2, 0, 4]);
        mixer.mix_one();
        assert_eq!(mixer.result(), vec![1, -3, 2, 3, -2, 0, 4]);
        mixer.mix_one();
        assert_eq!(mixer.result(), vec![1, 2, 3, -2, -3, 0, 4]);
    }

    #[test]
    fn test_mix_all() {
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        let result = Mixer::new(input).mix_all();
        assert_eq!(result, vec![1, 2, -3, 4, 0, 3, -2]);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "3")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        todo!("unimplemented");
    }
}
