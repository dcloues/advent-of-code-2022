use std::{
    error::Error,
    iter::Peekable,
    str::{Chars, FromStr},
};

#[derive(Debug)]
struct Input(Vec<(Packet, Packet)>);

#[derive(Debug, PartialEq, Eq, Clone)]
enum Packet {
    Singleton(i64),
    List(Vec<Packet>),
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Singleton(s), Packet::Singleton(o)) => s.cmp(o),
            (Packet::Singleton(_), Packet::List(_)) => Packet::List(vec![self.clone()]).cmp(other),
            (Packet::List(_), Packet::Singleton(_)) => self.cmp(&Packet::List(vec![other.clone()])),
            (Packet::List(s), Packet::List(o)) => s.cmp(o),
        }
    }
}

impl FromStr for Packet {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokenizer = Tokenizer {
            input: s.chars().peekable(),
        };

        parse_packet(&mut tokenizer.peekable())
    }
}

impl FromStr for Input {
    type Err = Box<dyn Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse_input(s)
    }
}

// impl Ord for

#[derive(PartialEq, Eq)]
enum Token {
    ListStart,
    Separator,
    Item(String),
    ListEnd,
}

fn parse_input(s: &str) -> Result<Input, Box<dyn Error>> {
    let mut pairs: Vec<(Packet, Packet)> = vec![];
    let mut tokens = Tokenizer {
        input: s.chars().peekable(),
    }
    .peekable();
    while let Some(pair) = parse_pair(&mut tokens)? {
        pairs.push(pair);
    }
    Ok(Input(pairs))
}

fn parse_pair(
    tokens: &mut Peekable<Tokenizer>,
) -> Result<Option<(Packet, Packet)>, Box<dyn Error>> {
    if tokens.peek().is_none() {
        Ok(None)
    } else {
        Ok(Some((parse_packet(tokens)?, parse_packet(tokens)?)))
    }
}

fn parse_packet(tokens: &mut Peekable<Tokenizer>) -> Result<Packet, Box<dyn Error>> {
    match tokens.next() {
        Some(Token::ListStart) => parse_rest_list(tokens),
        Some(Token::ListEnd) => Err("unexpected ] token".into()),
        Some(Token::Item(s)) => Ok(Packet::Singleton(
            s.parse::<i64>()
                .map_err(|e| -> Box<dyn Error> { e.into() })?,
        )),
        Some(Token::Separator) => Err("unexpected , token".into()),
        None => Err("unexpected end of input".into()),
    }
}

fn parse_rest_list(tokens: &mut Peekable<Tokenizer>) -> Result<Packet, Box<dyn Error>> {
    let mut items: Vec<Packet> = vec![];
    loop {
        match tokens.peek() {
            Some(Token::ListEnd) => {
                tokens.next(); // consume
                return Ok(Packet::List(items));
            }
            Some(_) => {
                items.push(parse_packet(tokens)?);
                tokens.next_if_eq(&Token::Separator);
            }
            None => return Err("unexpected end of input".into()),
        }
    }
}

struct Tokenizer<'a> {
    input: Peekable<Chars<'a>>,
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        // skip whitespace
        while let Some(_) = self.input.next_if(|c| c.is_whitespace()) {}
        match self.input.next() {
            Some('[') => Some(Token::ListStart),
            Some(']') => Some(Token::ListEnd),
            Some(',') => Some(Token::Separator),
            Some(ch) => {
                let mut buf = vec![ch];
                while let Some(ch) = self.input.next_if(|c| c.is_numeric()) {
                    buf.push(ch);
                }
                Some(Token::Item(buf.iter().collect()))
            }
            None => None,
        }
    }
}

pub fn part1(input: &str) -> Result<String, Box<dyn Error>> {
    let input: Input = input.parse()?;

    let correct = input
        .0
        .iter()
        .enumerate()
        .filter_map(|(i, (p1, p2))| (p1 <= p2).then_some(i as i64 + 1))
        .sum::<i64>();

    Ok(correct.to_string())
}

pub fn part2(input: &str) -> Result<String, Box<dyn Error>> {
    todo!("unimplemented")
}

#[cfg(test)]
mod test {
    use super::*;

    const INPUT: &str = include_str!("tests/day13test.txt");

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT).unwrap(), "13")
    }

    fn test_part2() {
        assert_eq!(part2(INPUT).unwrap(), "")
    }
}
