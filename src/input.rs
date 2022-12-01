#![allow(dead_code)]
use std::io::{stdin, BufReader, Read};
use std::str::FromStr;

pub struct Input <I> where I : Iterator<Item = char> {
    input : I,
    string : String
}

//TODO make this be in the impl block
pub fn input_new() -> Input<impl Iterator<Item = char>> {
    Input{input : BufReader::new(stdin()).bytes().map(|x| x.unwrap() as char), string : String::new()}
}

impl <I> Input<I> where I : Iterator<Item = char> {
    pub fn next<T>(&mut self) -> T where T : FromStr {
        self.string.clear();

        loop {
            let c = self.input.next().unwrap();
            if !c.is_whitespace() {
                self.string.push(c);
            }
            else if self.string.len() > 0 {
                break;
            }
        }

        let val : T = match self.string.parse() {Ok(val) => val, _ => panic!("Parse failure")};

        val
    }

    pub fn next_on_line<T>(&mut self) -> Option<T> where T : FromStr {
        if self.string == "\n" {
            self.string.clear();
            return None
        }

        self.string.clear();

        let mut c;
        loop {
            c = self.input.next().unwrap();
            if !c.is_whitespace() {
                self.string.push(c);
            }
            else if self.string.len() > 0 {
                break
            }
            else if c == '\n' {
                return None
            }
        }
        let val : T = match self.string.parse() {Ok(val) => val, _ => panic!("Parse failure")};

        if c == '\n' {
            self.string.clear();
            self.string.push('\n');
        }
        return Some(val)
    }

    pub fn next_char(&mut self) -> char {
        self.input.next().unwrap()
    }

    pub fn next_char_wsf(&mut self) -> char { //WSF = Whitespace free
        loop {
            let c = self.input.next().unwrap();
            if !c.is_whitespace() {break c;}
        }
    }
}
