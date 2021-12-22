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

        self.string.clear();

        val
    }
}