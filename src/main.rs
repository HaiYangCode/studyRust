use std::cmp::Ordering;
use std::io;
use rand::Rng;// use 导入非标准库

fn main() {
    println!("Guess the number!");



    let secret_number = rand::thread_rng().gen_range(0..101);


    loop {
        println!("Please input your guess.");
        let mut guess = String::new();
        println!("You guessed:{}", guess);
        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");
        // let guess: u32 = guess.trim().parse().expect("please type a number");

        let guess: u32 = match guess.trim().parse()//这里的match很关键
        {
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("small"),
            Ordering::Greater => println!("Greater"),
            Ordering::Equal => {
                println!("Equal");
                break;
            }
        }
    }
}
