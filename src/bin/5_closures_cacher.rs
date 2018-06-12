#![allow(unused_imports)]

use std::collections::HashMap;
extern crate rand;
use rand::Rng;
use std::thread;
use std::time::Duration;

struct Cacher<T> where T: Fn(u32) -> u32 {
    calculation: T,
    values: HashMap<u32, u32>,
}

impl<T> Cacher<T> where T: Fn(u32) -> u32 {
    
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            values: HashMap::new()
        }
    }
    fn value(&mut self, arg: u32) -> u32 {
        match self.values.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.values.insert(arg, v);
                v
            }
        }
    }
}

fn main() {
    let mut cacher = Cacher::new(|num| {
        println!("intensive calculation on input {:?}", num);
        thread::sleep(Duration::from_secs(1));
        num + 1
    }); 

    let secret_number = 2; // rand::thread_rng().gen_range(1, 11);
    println!("secret_number = {}", secret_number);

    if secret_number <= 5 {
        println!("Very computation intensive branch");
        cacher.value(secret_number);
        cacher.value(secret_number + 1);
        cacher.value(secret_number);
    } 
    if secret_number % 2 == 0 {
        println!("Even branch");
        cacher.value(secret_number);
    }
     
}