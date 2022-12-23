use std::{cell::RefCell, error::Error, rc::Rc};

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

#[derive(Eq)]
struct Segment {
    id: usize,
    datum: i32,
    previous: Option<Rc<RefCell<Segment>>>,
    next: Option<Rc<RefCell<Segment>>>,
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id && self.datum == other.datum
    }
}

struct Mixer {
    input: Box<Vec<i32>>,
    current: usize,
    list: Vec<Rc<RefCell<Segment>>>,
    head: Rc<RefCell<Segment>>,
}

impl Mixer {
    fn new(input: Vec<i32>) -> Self {
        let list = Self::build_segments(&input);
        let head = list[0].clone();
        let input = Box::new(input);
        Self {
            input,
            current: 0,
            list,
            head,
        }
    }

    fn build_segments(input: &Vec<i32>) -> Vec<Rc<RefCell<Segment>>> {
        let segments: Vec<Rc<RefCell<Segment>>> = input
            .iter()
            .enumerate()
            .map(|(i, d)| {
                Rc::new(RefCell::new(Segment {
                    id: i,
                    datum: *d,
                    next: None,
                    previous: None,
                }))
            })
            .collect();

        let head = segments[0].clone();
        let tail = segments[1].clone();
        head.borrow_mut().set_previous(tail.clone());
        tail.borrow_mut().set_next(head.clone());

        for window in segments.windows(2) {
            if let [a, b] = window {
                a.borrow_mut().next = Some(b.clone());
                b.borrow_mut().previous = Some(a.clone());
            } else {
                panic!("unexpected match failure");
            }
        }
        segments
    }

    fn mix_one(&mut self) -> Option<()> {
        self.list.get(self.current).and_then(|seg| {
            // TODO special handling if seg is the head!
            let distance = self.input[self.current];
            if distance > 0 {
                if *seg == self.head {
                    self.head = seg.borrow().next();
                }

                let new_previous = seg.borrow().nth(distance).clone();
                seg.borrow_mut().remove();
                seg.borrow_mut()
                    .set_next(new_previous.borrow().next().clone());
                new_previous
                    .borrow()
                    .next()
                    .borrow_mut()
                    .set_previous(seg.clone());
                new_previous.borrow_mut().set_next(seg.clone());
                seg.borrow_mut().set_previous(new_previous);
            }

            self.current += 1;
            Some(())
        })
    }

    // fn result<'a>(&'a self) -> Box<dyn Iterator<Item = i32> + 'a> {
    //     let mut current = self.head.clone();
    //     let mut done = false;
    //     Box::new(std::iter::from_fn(|| {
    //         if done {
    //             None
    //         } else {
    //             let cs = current.borrow();
    //             let value = cs.datum;
    //             current = cs.next();
    //             done = current == self.head;
    //             Some(value)
    //         }
    //     }))
    // }

    fn result(&self) -> Vec<i32> {
        let mut current = self.head.clone();
        let mut data = Vec::with_capacity(self.input.len());
        loop {
            data.push(current.borrow().datum);
            current = {
                let next = current.borrow().next();
                next
            };
            if current == self.head {
                break;
            }
        }

        data
    }
}

impl Segment {
    fn nth(&self, n: i32) -> Rc<RefCell<Segment>> {
        debug_assert!(n != 0);
        match n {
            1 => self.next(),
            -1 => self.previous(),
            _ if n > 1 => self.next().borrow().nth(n - 1),
            _ if n < 1 => self.previous().borrow().nth(n + 1),
            _ => panic!("invalid nth {n}"),
        }
    }

    fn set_next(&mut self, next: Rc<RefCell<Segment>>) {
        self.next = Some(next);
    }

    fn set_previous(&mut self, previous: Rc<RefCell<Segment>>) {
        self.previous = Some(previous);
    }

    fn remove(&self) {
        self.previous().borrow_mut().set_next(self.next());
        self.next().borrow_mut().set_previous(self.previous());
    }

    fn next(&self) -> Rc<RefCell<Segment>> {
        self.next.as_ref().unwrap().clone()
    }

    fn previous(&self) -> Rc<RefCell<Segment>> {
        self.previous.as_ref().unwrap().clone()
    }
}

pub fn part1(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

pub fn part2(_input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day7test.txt");

    #[test]
    fn test_mix() {
        let input = vec![1, 2, -3, 3, -2, 0, 4];
        let mut mixer = Mixer::new(input);
        mixer.mix_one();
        assert_eq!(mixer.result(), vec![2, 1, -3, 3, -2, 0, 4]);
    }

    #[test]
    #[ignore]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "3")
    }

    #[test]
    #[ignore]
    fn test_part2() {
        todo!("unimplemented");
    }
}
