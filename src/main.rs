use rand::random_range;

pub fn rand_num() -> i32 {
    let number = random_range(1..6);
    return number;
}

fn main() {
    let number = rand_num();

    if number == 1 {
        println!("You ROLLED: {:?}", number);
    } else if number == 2 {
        println!("You ROLLED: {:?}", number);
    } else if number == 3 {
        println!("You ROLLED: {:?}", number);
    } else if number == 4 {
        println!("You ROLLED: {:?}", number);
    } else if number == 5 {
        println!("You ROLLED: {:?}", number);
    } else {
        println!("You ROLLED: {:?}", number);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let number = rand_num();
        assert_eq!(number, 4);
    }
}
