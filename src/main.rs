extern crate core;


pub mod array;

fn main() {
    println!("Hello, world!");
    let res = array::is_unique_chars_ascii("hell0worldd");
    println!("{}",res);
   println!("{}",array::is_permutation_by_sort("mad","Dam"));
    println!("{}",array::is_permutation_by_count("mad","dam"));
    let string = array::urlify("hello world", 11);
    println!("{}",string)

}


