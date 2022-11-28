use std::cmp::Ordering;
use std::io;
use rand::Rng;// use 导入非标准库

fn main() {
    part6()
}

fn part6() {
    let four = IpAddressKind::V4;
    route(&four);
    let six = IpAddressKind::V6;
    let ipv4 = IpAdd::V4(String::from("xxx"), 42);
    match four {
        IpAddressKind::V4 => {
            println!("four")
        }

        IpAddressKind::V6 => {
            println!("six")
        }
    }
    // println!("value = {}", ipv4.0);
    // println!("value = {}", ipv4.1);
    let opt = Option::Some("");
    match ipv4 {
        IpAdd::V4(i, v) => {
            println!("value = {}", v);
        }
        IpAdd::V6(i, v) => {
            println!("value = {}", v);
        }
    }
}


fn route(ip_type: &IpAddressKind) {}

enum IpAddressKind {
    V4,
    V6,
}

enum IpAdd {
    V4(String, u32),
    V6(String, u32),
}

fn part5() {
    let rec = Rectangle::newRectangle(20, 40);
    Rectangle::newMethod();
    let rec01 = Rectangle {
        width: 20,
        height: 30,
    };
    // Rectangle::newMethod2(&rec01);
    let result = Rectangle::area(&rec01);
    println!("result == {}", result)
}

struct Rectangle {
    width: u32,
    height: u32,

}


impl Rectangle {
    fn newMethod() {}
    fn newMethod2(&self) {}
    fn newRectangle(width: u32, height: u32) -> Rectangle {
        Rectangle {
            width: width,
            height: height,
        }
    }
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn part4() {
    let x = String::from("Hello");
    let s2 = x.clone();
    println!("s2 = {} , x ={}", s2, x);
    // let person: Person = Person::new();
}

fn Person() -> String {
    todo!()
}

fn game() {
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
                println!("You are win,value is :{}", &secret_number);
                break;
            }
        }
    }
}


