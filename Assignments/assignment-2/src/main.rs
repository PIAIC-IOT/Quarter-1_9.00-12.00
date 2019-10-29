/* Assignment key by Muhammad Jamshaid tahiri*/
// Line comments which go to the end of the line.
/* Block comments which go to the closing delimiter. */
use std::io;
fn main() {

/* PART-1 Q1 

let x = 3;
match x {
3 => println!("ndbf {}",x), //use comma instead of semicolon
_ => {},                    // this is for rust compiler to match all possible cases
}
*/
/* PART-1 Q2 

let mut x = 0;  //use mut 
loop{
println!("Hello {}", x);

if x == 100{            // change the condition from 10 to 100 or 101 according to code logic
    break;
}
x+=1;                  //this was shifted after if block
}
*/

/* PART-1 Q3  

let x = 3;
loop {
println!("I love Stranger things");
if x==3 {   //use "==" instead of "=" to compare
println!("I don’t love it anymore!");
break;  //use break to end this loop
}
}
*/

/* PART-1 Q4  
let number1:i32 = 12;
let number2:i32 = 12;
if number1 == number1 {
println!("Number1 and Number2 are equal\n");    //there was logic error so change number =! to number ==
}else{
println!("Number1 and Number2 are not equal\n");}
*/
/* PART-1 Q5  
let mut x = 5;
loop {
x += x -- 3;
println!("{}", x);
if x % 5 == 0 {    //error with = sign as this operator will set value whereas == will check conditional arguments
    break; 
    }
}
*/
/* PART-1 Q6  
let mut sum=0;
for i in 0..10 {
let mut data = String::new();
    println!("Enter integer {}",i+1);
    io::stdin().read_line(&mut data);  // In read_line argument must be address
    let data:i32 = data.trim().parse().unwrap(); //parsed data must explicity be defined
    sum = sum + data;
}
println!("The sum is: {}",sum);
println!("The average is: {}",sum/10);
*/

/* PART-1 Q7  

// display the cube of the number upto a given integer.
let x = 3;
for i in 0..x+1{                                                                           //add +1 to x
println!("Number is :{} and cube is :{}",i,(i as i32).pow(3) ); //pow is method for
}                                                                                         //add parentheses } to end for loop      
// power                                                                                 //comment this expression
// (num as datatype).pow(power)
*/
/* PART-1 Q8 

let names = ["Ali", "Zain" ,"Naufil"];  // add , between 2 names and semicolona t end
for name in names.iter() {
match name {
&"Ali" => println!("There is a rockstar among us!"),
_ => println!("Hello {}", name),
}
}                                   // add parentheses at the end to end for loop
 */
/* PART-1 Q9  

let array: [i32; 5] = [8, 9, 3, 4, 5];
let mut sum = 0;                    // add mut to sum
println!("Find sum of all elements of array:");
println!("----------------------------------");
for n in 0..5 {                 //remove range and change 1 to 0
sum += array[n];
}
println!("Sum of all elements stored in the array is : {}", sum);
*/
/* PART-1 Q10 

let numbers = [20, 30, 25, 35, 16, 60, 100];
//calculate sum of all array elements
let mut sum : i32 = 0;              // add let mut and i32
for a in 0..numbers.len() {
sum = sum + numbers[a];             //change from i to a
}
//calculate average value
let average = sum / (numbers.len() as i32);     // type cast 
println!("Average value of the array elements is : {}" , average);  // parentheses complete{}
 */
/* PART-1 Q11 
let cost_price = 12;        //add semicolon
let sale_price = 13;
let mut profit_lost = 0;    //add let keyword

if sale_price>cost_price //calculate profit
{
profit_lost = sale_price-cost_price;
println!("You can booked your profit amount : {}", profit_lost);
}
else if (cost_price>sale_price) //calculate loss
{
profit_lost = cost_price-sale_price;
println!("You got a loss of amount : {}", profit_lost);
}
else //No Profit No Loss
{
println!("You are running in no profit no loss condition.");
}
*/




//<<<PART 2>>>
/* PART-2 Q1 

let mut input = String::new();
io::stdin().read_line(&mut input);
let input : i32 = input.trim().parse().unwrap();
let mut c = 0;
for i in 1..input+1{
    if input % i == 0{
        c+=1
    }
}
if c == 2{
    print!("the number is prime");
}
else {
    print!("the number is not prime");
}
*/

/* PART-2 Q2 
let mut c =0;
loop {
    println!("I love my mother");
    c+=1;
    if c ==3 {
        println!("I love my Father");
        break;
    }
}
*/

/* PART-2 Q3

let input =100;
match input {
    10 => println!("Decade"),
    100 => println!("Century"),
    1000=> println!("Millenium"),
    _=> println!("Please enter 10,100 or 1000"),
}
*/
/* PART-2 Q4

let mut sum = 0;
for i in 1..11{
    sum+=i
}
println!("The sum is : {}",sum);
*/

/* PART-2 Q5

let mut Math_marks = String::new();
io::stdin().read_line(&mut Math_marks);
let Math_marks : i32= Math_marks.trim().parse().unwrap();

let mut Physics_marks = String::new();
io::stdin().read_line(&mut Physics_marks);
let Physics_marks : i32= Physics_marks.trim().parse().unwrap();

let mut Chemistry_marks = String::new();
io::stdin().read_line(&mut Chemistry_marks);
let Chemistry_marks : i32= Chemistry_marks.trim().parse().unwrap();

let Total = Math_marks+Physics_marks+Chemistry_marks;

if Math_marks >=65 && Physics_marks>=55 && Chemistry_marks >=50 && Total >=180{
    println!("You are eligible");
}
else {
    println!("You are not eligible");
}
*/
}
