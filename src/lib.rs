pub mod input;
pub use input::*;

#[cfg(test)]
mod tests {
    use crate::input::*;

    #[test]
    fn next_test() {
        let mut input = input_new();

        let x : u64 = input.next();
        let y : i32 = input.next();
        let s : String = input.next();

        println!("X : {}", x);
        println!("Y : {}", y);
        println!("S : {}", s);
    }
}
