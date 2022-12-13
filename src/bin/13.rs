use std::cmp::{Eq, Ord, Ordering, PartialEq, PartialOrd};
use std::fs;

#[derive(Debug)]
enum TokenType {
    EOF = 0,
    LBracket,
    RBracket,
    Comma,
    Int,
}

#[derive(Debug)]
struct Token {
    pub typ: TokenType,
    pub pos: usize,
    pub value: String,
}

#[derive(Debug)]
struct Lexer {
    input: Vec<char>,
    start: usize,
    pos: usize,
}

const EOF: char = 0 as char;

impl Lexer {
    fn new(input: &str) -> Lexer {
        Lexer {
            input: input.chars().collect(),
            start: 0,
            pos: usize::MAX,
        }
    }

    fn next(&mut self) -> char {
        self.pos = if self.pos == usize::MAX {
            0
        } else {
            self.pos + 1
        };
        if self.pos < self.input.len() {
            return self.input[self.pos];
        }
        EOF
    }

    fn backup(&mut self) {
        self.pos -= 1;
    }

    fn accept(&mut self, test: fn(c: char) -> bool) -> bool {
        if test(self.next()) {
            return true;
        }
        self.backup();
        false
    }

    fn emit(&mut self, typ: TokenType) -> Option<Token> {
        let res = Some(Token {
            typ,
            pos: self.start,
            value: String::from_iter(self.input[self.start..self.pos + 1].iter()),
        });
        self.start = self.pos + 1;
        res
    }

    fn next_token(&mut self) -> Option<Token> {
        let c = self.next();
        match c {
            EOF => {
                self.backup();
                self.emit(TokenType::EOF)
            }
            '[' => self.emit(TokenType::LBracket),
            ']' => self.emit(TokenType::RBracket),
            ',' => self.emit(TokenType::Comma),
            _ => {
                if c.is_numeric() {
                    while self.accept(|c| c.is_numeric()) {}
                    return self.emit(TokenType::Int);
                }
                None
            }
        }
    }
}

#[derive(Debug, Eq, Clone)]
enum Segment {
    List { segments: Vec<Segment> },
    Int { value: i32 },
}

impl Ord for Segment {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Segment::Int { value } => match other {
                Segment::Int { value: other_value } => value.cmp(other_value),
                Segment::List { segments: _ } => {
                    let t = Segment::List {
                        segments: vec![self.clone()],
                    };
                    t.cmp(other)
                }
            },
            Segment::List { segments } => match other {
                Segment::Int { value: _ } => {
                    let t = Segment::List {
                        segments: vec![other.clone()],
                    };
                    self.cmp(&t)
                }
                Segment::List {
                    segments: other_segments,
                } => {
                    for i in 0..segments.len().min(other_segments.len()) {
                        let c = segments[i].cmp(&other_segments[i]);
                        if !c.is_eq() {
                            return c;
                        }
                    }
                    segments.len().cmp(&other_segments.len())
                }
            },
        }
    }
}

impl PartialOrd for Segment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Segment {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Segment::Int { value } => match other {
                Segment::Int { value: other_value } => value == other_value,
                Segment::List { segments: _ } => {
                    let t = Segment::List {
                        segments: vec![self.clone()],
                    };
                    t == *other
                }
            },
            Segment::List { segments } => match other {
                Segment::Int { value: _ } => {
                    let t = Segment::List {
                        segments: vec![other.clone()],
                    };
                    *self == t
                }
                Segment::List {
                    segments: other_segments,
                } => {
                    for i in 0..segments.len().min(other_segments.len()) {
                        if segments[i] != other_segments[i] {
                            return false;
                        }
                    }
                    segments.len() == other_segments.len()
                }
            },
        }
    }
}

#[derive(Debug)]
struct Parser {
    lex: Lexer,
    typ: TokenType,
    pos: usize,
    value: String,
}

impl Parser {
    fn new(lex: Lexer) -> Parser {
        Parser {
            lex,
            typ: TokenType::EOF,
            pos: 0,
            value: "".to_string(),
        }
    }

    fn next(&mut self) {
        let tok = self.lex.next_token().unwrap();
        self.typ = tok.typ;
        self.pos = tok.pos;
        self.value = tok.value;
    }

    fn parse_list(&mut self) -> Result<Segment, &'static str> {
        let mut segments = vec![];
        self.next();
        loop {
            match self.typ {
                TokenType::EOF => return Err("unexpected EOF"),
                TokenType::LBracket => segments.push(self.parse_list().unwrap()),
                TokenType::RBracket => {
                    self.next();
                    return Ok(Segment::List { segments });
                }
                TokenType::Int => segments.push(self.parse_int().unwrap()),
                _ => self.next(),
            }
        }
    }

    fn parse_int(&mut self) -> Result<Segment, &'static str> {
        let value = self.value.parse::<i32>().unwrap();
        self.next();
        Ok(Segment::Int { value })
    }

    fn parse(&mut self) -> Result<Segment, &'static str> {
        self.next();
        return match self.typ {
            TokenType::LBracket => self.parse_list(),
            _ => Err("unexpected input"),
        };
    }
}

fn main() {
    let input = fs::read_to_string("./inputs/13.txt").unwrap();
    let packets = input.trim().split("\n\n").map(|t| {
        let (l, r) = t.split_once('\n').unwrap();
        let l = Parser::new(Lexer::new(l)).parse();
        let r = Parser::new(Lexer::new(r)).parse();
        l.cmp(&r)
    });

    let part1_sum = packets
        .enumerate()
        .filter_map(|(i, ord)| {
            if ord.is_lt() {
                return Some(i + 1);
            }
            None
        })
        .sum::<usize>();

    println!("part1: {:?}", part1_sum);

    let divider_packet_0 = Segment::List {
        segments: vec![Segment::List {
            segments: vec![Segment::Int { value: 2 }],
        }],
    };
    let divider_packet_1 = Segment::List {
        segments: vec![Segment::List {
            segments: vec![Segment::Int { value: 6 }],
        }],
    };

    let mut packets = input
        .split("\n")
        .filter(|t| !t.is_empty())
        .map(|t| Parser::new(Lexer::new(t)).parse().unwrap())
        .collect::<Vec<_>>();
    packets.push(divider_packet_0.clone());
    packets.push(divider_packet_1.clone());
    packets.sort();

    let mut part2_value = 1;
    for (i, p) in packets.iter().enumerate() {
        if p == &divider_packet_0 || p == &divider_packet_1 {
            part2_value *= i + 1
        }
    }

    println!("part2: {:?}", part2_value);
}
