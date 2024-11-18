#![allow(dead_code, unused_variables)]

use rand::{thread_rng, Rng};

fn add(i: i32, j: i32) -> i32 {
    i + j
}

fn div(i: i32, j: i32) -> i32 {
    if j == 0 {
        0
    } else {
        i / j
    }
}

const ANSWER_OF_EVERYTHING: i32 = 42;

const ANOTHER_ANSWER: i32 = ANSWER_OF_EVERYTHING + 1;
const ANOTHER_ANSWER_2: i32 = calc_another_anwer(ANSWER_OF_EVERYTHING);

const fn calc_another_anwer(original_answer: i32) -> i32 {
    original_answer + 1
}

fn main() {
    println!("Hello, world!");

    let a = 10;
    let mut b = 2;

    println!("{}", add(a, b));
    println!("{}", div(a, b));

    b += 10;
    println!("{}", add(a, b));

    {
        let b = b;
        //b += 20;
    }

    let a;
    if b > 10 {
        a = 10;
    } else {
        a = 20;
    }

    println!("{}", a);

    let random_number = thread_rng().gen_range(1..=10);

    // 1..4 -> Lost
    // 5 -> Nearly Win
    // 6..10 -> Win
    let result = match random_number {
        n if n >= 6 => "Win",
        5 => "Nearly Win",
        _ => "Lost",
    };
    println!("{}", result);

    let numbers = [10; 5]; // Stack
    
    let mut numbers = vec![1, 2, 3, 4, 5]; // Heap
    numbers.push(6);
    
    for n in numbers {
        println!("{}", n + 1);
    }
}
