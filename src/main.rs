extern crate rand;
use std::io;

use rand::thread_rng;
use rand::Rng;

fn main() {
    let mut line = String::new();
    println!("Enter your name :");
    io::stdin().read_line(&mut line).expect("Failed to read input");
    println!("Hello {} Welcome to Rock Paper Scissor Game", line);
    let x = number();
    let mut uchoose = String::new();
    println!("{} now Choose 1.Rock, 2.Paper, 3.Scissor", line);
    io::stdin().read_line(&mut uchoose).expect("Failed to read input");
    let user: u8 = uchoose.trim().parse().expect("Not a valid number");

    println!("Computer choose {}", x);
    println!("You choose {}", user);

    if user == x {
        println!("tie")
    }
    
    if user == 1 && x == 2 {
        println!("You lose computer wins!")
    } if user == 1 && x == 3 {
        println!("You win computer lose!")
    } if user == 2 && x == 3 {
        println!("You lose computer wins!")
    } if user == 2 && x == 1 {
        println!("You win computer lose!")
    } if user == 3 && x == 2 {
        println!("You win computer lose!")
    } if user == 3 && x == 1 {
        println!("You lose computer wins!")
    }
}

fn number() -> u8 {
    let mut rng = thread_rng();
    let i: u8 = rng.gen_range(1, 4);
    return i
}