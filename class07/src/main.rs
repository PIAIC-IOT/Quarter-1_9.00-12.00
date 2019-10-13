// #[macro_use] 
// extern crate text_io;
// fn main() {
//     let a: i32 = read!();
//     let b: i32 = read!();
//     let c: i32 = read!();    
//     let s = (a+b+c)/2;
//     let r = (s*(s-a)*(s-b)*(s-c)) as f64; 
//     let final_value = r.sqrt();   
//     println!("The Result is {},{},{}", s,r,final_value);
// }


// fn main() {
//     let t:(i32,i32,i32) = (23,43,54);
//     let s = (t.0+t.1+t.2)/2;    
//     let r = (s*(s-t.0)*(s-t.1)*(s-t.2)) as f64; 
//     let final_value = r.sqrt();   
//     println!("The Result is {} {} {}", s,r,final_value);
// }

// #[derive(Debug)]
// struct Student{
//     first_name: String,
//     last_name: String,
//     age: i8,
//     single: bool,
// 	height: f32, 
// }
// fn main(){
//     let student_01 = Student{
//         first_name: "Faheem".to_string(),
//         last_name:"Uz Zaman".to_string(),
//         age: 24,
//         single: true,
//         height: 5.8,
//     };    
// }
// impl Student{
//     fn student_information(self) -> String{
//         let student = format!("Student Name:{} {}\nStudent Age:{}\nStudent Height:{}",
//         self.first_name,self.last_name,self.age,self.height);
//         student        
//     }    
// }

// #[derive(Debug)]
// struct Area{
//     a: i32,
//     b: i32,
//     c: i32,
// }

// impl Area {
//     fn caluclate_area(&self) -> f64{
//         let s = (self.a + self.b + self.c)/2; // BODMAS: Bracket Divide Multiply Addition Subtraction
//         let r = (s*(s-self.a)*(s-self.b)*(s-self.c)) as f64; 
//         let final_value = r.sqrt();
//         final_value
//     }
// }
// fn main(){
//     let area_01 = Area{a:23,b:43,c:54};
//     let area_02 = Area{a:10,b:40,c:62};
//     println!("{}", area_01.caluclate_area());
//     println!("{:?}", area_01);
// }

// Write a Program which calculate the Area of Rectangle lxw
// Where l =length and w=width and calculate this by using method
// which is calculate the area_of_rectangele 
mod student;
use student::student_work;
fn main(){   
        let student_01 = student::student_work::Student{
        first_name: "Faheem".to_string(),
        last_name:"Uz Zaman".to_string(),
        age: 24,
        single: true,
        height: 5.8,
    }; 
    println!("{:?} {}", student_01,student_01.student_information());    
}
