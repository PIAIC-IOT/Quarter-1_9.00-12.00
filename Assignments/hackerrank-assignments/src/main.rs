/* Q1  Hello World
fn main(){
println!("Hello, World!");
}
*/


/* Q2 Function


use std::{io};

fn main(){
	
  	let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");

    let int: i32 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
    
    println!("{}",is_even_odd(int));
    // Enter your code below this line.
}

fn is_even_odd(x:i32)->String{
	// Enter your code here.
    if x%2 == 0{
        "Even".to_string()
    }
    else {
        "Odd".to_string()
    }
}
*/


/* Q3 Factorial


// Enter your code here 
use std::io;
fn main(){
    
    let mut input = String::new();
    io::stdin().read_line(&mut input);
    let input: i32 = input.trim().parse().unwrap();
    print!("{}",factorial(input));
}
fn factorial(input:i32)->i32{
    // for i in 0..(input+1){
    if input == 0{
        1
    }
    
    else {
        factorial(input-1)*input
    }
    // }
    
    
}
*/
/* Q4  Vector Ds-1
use std::{io};


fn main(){
    let mut int = String::new();
    io::stdin().read_line(&mut int)
        .expect("Failed to read your input");

    let int: i32 = match int.trim().parse() {
        Ok(num) => num,
        Err(_) => 0
    };
  
// Enter your code here.
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Failed to read your input");


 let mut emptyvec = vec![];
    for i in input.split_whitespace(){
    let x : i32 = i.parse().unwrap();
emptyvec.push(x);
    // println!("{}",i );
    }
    for i in emptyvec.iter().rev(){
print!("{} ",i);
    }
}
*/