use std::io;
fn main() {
    let mut x =5;
    println!(" The Value of X is {x}");

    x=6;
    println!("The value of X is {x}");

    let spaces = " ";
    let spaces = spaces.len();
    println!("{spaces}");

    // Working with arrays
    let a = [1,2,3,4,5];
    println!("Please enter an array index");

    let mut index = String::new();
    io::stdin()
    .read_line(&mut index)
    .expect("Failed to read line");

    let index: usize =index
    .trim()
    .parse()
    .expect("Index entered was not a number");
    
    let element = a[index];

    println!("The value of the element at index {index} is {element}");
    loops();
}

fn loops(){
    let mut counter =0;

    let result = loop {
        counter +=1;

        if counter == 10{
            break counter *2;
        }
    };

    println!("The result is {result}");
}