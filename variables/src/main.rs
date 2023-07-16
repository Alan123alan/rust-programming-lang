use std::io;

fn main() {
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
    let mut y = 10;
    println!("The value of y is: {y}");
    y = y * 3;
    println!("The value of y is: {y}");
    let tuple = (1,"hello",3.6);
    println!("The value of tuple is {}, {}, {}", tuple.0, tuple.1, tuple.2);
    //arrays are fixed in size and all elements in array must be same type
    let array: [i32;5] = [1, -1, 3, 5, 6];
    let mut index = String::new();
    println!("Please select an index from 0 to {}", array.len()-1);
    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");
    let index:usize = index.trim().parse().expect("Index entered was not a number");
    println!("You selected item at index {index}: {}",array[index]);
}
