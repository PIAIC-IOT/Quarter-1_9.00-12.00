// fn main() {
//     let x  = 10;
//     let y  = x;
//     println!("{} ,{}",x,y);

//     let s1 = String::from("Hello");
//     let s2 = s1.clone();
//     let s3 = &s1;

//     let s4 = &s1;
//     println!("{}, {}, {}, {}", s1, s2, s3, s4);
// // }
// fn main() {
//     let s = String::from("hello"); 
//     takes_ownership(s.clone()); 
//     println!("{}",s);            
//     let x = 5;                      
//     makes_copy(x);                  
// }
// fn takes_ownership(some_string: String) {
//     println!("{}", some_string);
// } 
// fn makes_copy(some_integer: i32) { 
//     println!("{}", some_integer);
// }

fn main() {
    let s1 = gives_ownership();
    let s2 = String::from("hello");    
    let s3 = takes_and_gives_back(s2);
    println!("{}",s1);    
        println!("{}",s3);        
    

}
fn gives_ownership() -> String {            
    let some_string = String::from("hello");
    some_string                             
}
fn takes_and_gives_back(a_string: String) -> String { 
    a_string  
}
