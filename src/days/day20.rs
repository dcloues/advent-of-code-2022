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
    datum: i64,
    previous: Cell<ID>,
    next: Cell<ID>,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.datum == other.datum
    }
}

struct Mixer {
    input: Vec<i64>,
    current: usize,
    list: Vec<Segment>,
    head: ID,
}

impl Mixer {
    fn new(input: Vec<i64>) -> Self {
        let list = Self::build_segments(&input);
        Self {
            input,
            current: 0,
            list,
            head: 0,
        }
    }

    fn build_segments(input: &Vec<i64>) -> Vec<Segment> {
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

    fn mix_all(mut self) -> Vec<i64> {
        while let Some(_) = self.mix_one() {}
        self.result()
    }

    fn mix_all_n_times(mut self, n: i64) -> Vec<i64> {
        for _ in 0..n {
            while let Some(_) = self.mix_one() {}
            self.current = 0;
        }
        self.result()
    }

    fn mix_one(&mut self) -> Option<()> {
        if self.current >= self.input.len() {
            return None;
        }

        let current = &self.list[self.current];
        // println!(
        //     "Mixing id={} datum={} dec={} modulo_length={}",
        //     current.id,
        //     current.datum,
        //     current.datum / 811589153,
        //     current.datum % self.input.len() as i64
        // );
        let lenmod = (self.input.len() as i64) - 1;
        if current.datum.is_positive() {
            for _ in 0..(current.datum % lenmod as i64) {
                let swapped = current.move_right(self);
                if self.head == current.id {
                    self.head = swapped.id;
                }
            }
        } else if current.datum.is_negative() {
            for _ in 0..(current.datum.abs() % lenmod) {
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

    fn result(&self) -> Vec<i64> {
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

fn extract_coordinates(mixed: &Vec<i64>) -> i64 {
    let i = mixed.iter().position(|i| *i == 0).unwrap();
    let k1 = mixed[(i + 1000) % mixed.len()];
    let k2 = mixed[(i + 2000) % mixed.len()];
    let k3 = mixed[(i + 3000) % mixed.len()];
    println!("i={i} k1={k1}, k2={k2}, k3={k3}");
    k1 + k2 + k3
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let input: Vec<i64> = input.lines().map(|s| s.parse()).collect::<Result<_, _>>()?;
    let result = Mixer::new(input).mix_all();
    Ok(extract_coordinates(&result).to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    let mut input: Vec<i64> = input.lines().map(|s| s.parse()).collect::<Result<_, _>>()?;
    input.iter_mut().for_each(|i| *i *= 811589153);

    let result = Mixer::new(input).mix_all_n_times(10);
    Ok(extract_coordinates(&result).to_string())
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
    fn test_mix_large_n() {
        let input = vec![
            811589153,
            1623178306,
            -2434767459,
            2434767459,
            -1623178306,
            0,
            3246356612,
        ];

        let result = Mixer::new(input).mix_all_n_times(10);
        assert_eq!(
            result,
            vec![
                0,
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153
            ]
        );
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "3")
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "1623178306")
    }
}
