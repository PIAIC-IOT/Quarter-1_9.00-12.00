// fn main() {
//     println!("The Sum is: {}", add_two_number(45, 30)); // Function Calling

//     println!("The Square is: {}", square(25)); 

//     hello_world();

//     print_name("Faheem".to_string());
//     println!("The Square is: {}",calculate_square());

//     let mut input_01 = String::new();
//     io::stdin().read_line(&mut input_01);
//     let input_01:i32 = input_01.trim().parse().unwrap();
//     let mut input_02 = String::new();
//     io::stdin().read_line(&mut input_02);
//     let input_02:i32 = input_02.trim().parse().unwrap();

//     println!("The greater input number is {}", check_maxV2(input_01, input_02));

//     let cal_result = arthimatic_operations(input_01, input_02);
//     println!("The Sum is: {}",cal_result.0);
//     println!("The Sub is: {}",cal_result.1);
//     println!("The Mul is: {}",cal_result.2);
//     println!("The Div is: {}",cal_result.3);

//     let cal_sq_cb = square_and_cube(input_01);
//     println!("The number {} square is {} and cube is {}",
//     input_01,cal_sq_cb.0,cal_sq_cb.1);

//     let arrNum = [345,567,678,234,456,23];
//     println!("The Max Number Is {}",largest(&arrNum));

//     let arrNum02 = [345,567,6783,234,456,23];
//     println!("The Max Number Is {}",largest(&arrNum02));
    
// }

// // this function taking the two interger parameter and return the sum of the two numbers.
// // // addTwoNumber function Name
// // // (x:i32,y:i32) Function Paramerters
// // // ->i32 indicates Return Type
// // //  sum
// // // Function Decalartion

// fn add_two_number(x:i32,y:i32)->i32{
//     let sum:i32;
//     sum = x+y;
//     sum
// }

// // this function get the input parameter as integer and return the square of given number.
// fn square(x:i32)->i32{
//     x*x
// }
// // this function just print the single line statement.
// fn hello_world(){
//     println!("Hello World");
// }

// // this function take one parameter as string and print it.
// fn print_name(name:String){
//     println!("The name is: {}",name);
// }

// // this function calculate the square on behalf of user dynamic input.
// use std::io;
// fn calculate_square()->i32{
//     let mut input = String::new();
//     io::stdin().read_line(&mut input);
//     let input:i32 = input.trim().parse().unwrap();
//     input*input
// }

// // this function takes two input parameters and check which one is the largerst and print the larger one.
// fn check_max(x:i32,y:i32)->String{
//     if x>y{
//         "The number 01 input is greater".to_string()
//     }
//     else{
//         "The number 02 input is greater".to_string()
//     }
// }
// // this function takes two input parameters and check which one is the largerst and return the larger one.
// fn check_maxV2(x:i32,y:i32)->i32{
//     if x>y{
//         x
//     }
//     else{
//         y
//     }
// }

// // this function take the first name and last name using the dynamic input from the user
// // and return the tuple as first name and lastname.
// fn take_full_name()->(String,String){

//     println!("Enter First Name");
//     let mut input_01 = String::new();
//     io::stdin().read_line(&mut input_01);
//     let input_01 = input_01.trim().to_string();

//     println!("Enter Last Name");
//     let mut input_02 = String::new();
//     io::stdin().read_line(&mut input_02);
//     let input_02 = input_02.trim().to_string();

//     (input_01,input_02) // return the tuple
// }

// // this function take one integer input (i32) and return the all mathematical operation addition, subtraction,
// //  multiplication and division.
// fn arthimatic_operations(x:i32,y:i32)->(i32,i32,i32,i32){
//     (x+y,x-y,x*y,x/y)
// }

// // this function take one integer input (i32) and return the square and cube in the form of tuple
// fn square_and_cube(x:i32)->(i32,i32){
//     (x*x,x*x*x)
// }

// // this function take input as integer (i32) array and return the largest value in array.
// fn largest(arrNum:&[i32])->i32{

//     let mut max = arrNum[0];
//     for x in (1..arrNum.len())
//     {
//         if max < arrNum[x]{
//             max = arrNum[x];
//         }
//     }
//     max
// }