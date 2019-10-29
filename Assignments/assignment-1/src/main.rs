
/* Assignment key by Muhammad Jamshaid tahiri*/
// Line comments which go to the end of the line.
/* Block comments which go to the closing delimiter. */
use std::io;
fn main() {

/* PART-1 Q1
       println!("Hello, world!"); //semicolon and macro(!) was missing
*/
  
  
/* PART-1 Q2
     let chocolate1 = 10;
    let chocolate2 = 10;
    let total: u32 = chocolate1 + chocolate2;      //can change cons to let
    println!("The sum of x and y is:{} ",total);   // {} parenthesis was missiong
*/

/* PART-1 Q3
    let x = 2.5;
    let y = 3;
    let z = x * y as f32;   //was trying to multiply different data types. you can type cast y as f32
    println!("{}",z);
*/

/*PART-1 Q4
   let radius = 6.0;
   let perimeter:i32;    

   perimeter = (2.0*3.14*radius) as i32; // this will calculate to floating points so //w e have to type cast it by using as i32 
                                        
   println!("Perimeter of the Circle = {} inches", perimeter);
*/	

/*PART-1 Q5

    let mut x = "haris";     //after shadowing variable doesn't need to be mutable so rustc will throw warning
    println!("{}",x);
    let x = x.len();         //here data type change from str to usize so we can shadow it simply by adding let
    println!("{}",x) ;
*/

/*PART-1 Q6

    let mut x = 3;
    println!("Number {}", x);
    x = 5;                       //variable was immutable so add mut to first variable also can shadow by using let
    println!("Number {}", x);

*/


/*PART-1 Q7

    // Short-circuiting boolean logic
    println!("true AND false is {}", true && false); //false spelling mistake ,and parenthesis and semicolon was missing
    println!("true OR false is {}", true || false);
    println!("NOT true is {}", !true);

*/


/*PART-1 Q8

   let interest:f32 = 8.0;  //f32 was defined while number was i32 type . you can change 8 to 8.0 or add (8.0 as f32) or even you can change data type from f32 to i32
   println!("interest is {}",interest);
*/

/*PART-1 Q9

let long_lived_binding = 1;
// This is a block, and has a smaller scope than the main function
//{
// This binding only exists in this block
let short_lived_binding = 2;
println!("inner short: {}", short_lived_binding);
println!("inner short: {}", long_lived_binding);
//} 
// Error! `short_lived_binding` doesn't exist in this scope
println!("outer short: {}", short_lived_binding); // this error can be remove by removing or commenting parenthesis of scope
// FIXME ^ Comment out this line
*/
  


/*PART-1 Q10


    let a_binding;
    {
        let x = 2;
        // Initialize the binding
        a_binding = x * x;
    }

    println!("a binding: {}", a_binding); //spelling mistake

*/

/* PART-2 Q1
let x =95.0;
let n = 150.0;
let p = x/n*100.0 ;
println!("Percentage in Chemistry is {}",p);
*/


/* PART-2 Q2

//(use std::io) this will be used before main function to take standard library in to scope

let mut x = String::new();
io::stdin().read_line(&mut x);

print!("{}",x);
*/

/* PART-2 Q3 

let name : String = String::from("MUHAMMAD JAMSHAID TAHIRI") ;
let age : i32 = 20;
let mobile_no : String = String::from("0900-78601");
println!("name :{} \nage:{} \nmobile no: {} ",name,age,mobile_no );
*/

/* PART-2 Q4 

let a : i32 =125;
let b : i32 = 12345;
let ax : i64 = 1234567890;
let s : i16 =4043;
let x : f32 = 2.13459;
let dx : f64 = 1.1415927;
let c : char = 'W';
let ux : u64 = 2541567890;
println!("a + c is {}",  a + c as i32); 
println!("x + c is {}",  x as i32 + c as i32); 
println!("dx + x is {}",  dx + x as f64);
println!("a + x is {}",  a + x as i32 );
println!("s + b is {}",  s + b as i16);
println!("ax + b is {}",  ax + b as i64);
println!("s + c is {}",  s as i32 + c as i32); 
println!("ax + c is {}",  ax as i32 + c as i32); 
println!("ax + ux is {}",  ax + ux as i64);
*/
/* PART-2 Q5 

let days = 1329;
let years = days/365;
let days = 1329-(365*years);
let week = days/(7);
println!("years :{} \nweeks: {}",years,week );

//also in this way
let days = 1329;
let years = days/365;
let week = days%365/7;

println!("years :{} \nweeks: {}",years,week );

*/

}