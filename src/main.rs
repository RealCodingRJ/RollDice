use core::panic;

use rand::random_range;

pub fn rand_num() -> Result<(), i32> {
    let number = random_range(1..6);

    if number == 4 {
        Ok(())
    } else {
        panic!("Not a Number")
    }
}

fn main() {
    let number = rand_num();

    if number == Err(1) {
        println!("You ROLLED: {:?}", number);
    } else if number == Err(2) {
        println!("You ROLLED: {:?}", number);
    } else if number == Err(3) {
        println!("You ROLLED: {:?}", number);
    } else if number == Err(4) {
        println!("You ROLLED: {:?}", number);
    } else if number == Err(5) {
        println!("You ROLLED: {:?}", number);
    } else {
        println!("You ROLLED: {:?}", number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() -> Result<(), i32> {
        let number = rand_num();

        match number {
            Ok(_) => Ok(()),
            Err(_) => todo!(),
        }
    }
}
