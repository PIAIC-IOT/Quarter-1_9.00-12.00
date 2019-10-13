// extern crate my_lib as my_package;
// fn main() {  
//     my_package::print_text();      
// }

// // // // lib.rs
// // pub fn print_text(){
// //     println!("Hello World From Main Package");
// // }

// Batch 02 IOT section 01(9 to 12)
// www.tiny.cc/IOTGS1

// // use iot_io::read_inputs as io;
// fn main(){
//     println!("Enter the name");
//     let name = io::read();
//     println!("The Name Is:{}", name);
// }

// extern crate student;

// use student::student_register as mod_s;
// use student::teacher_register as mod_t;
// // use student::{teacher_register,student_register};
// fn main(){
//     let student_01 = mod_s::Student::new("Faheem".to_string(),"Faheem@yahoo.com".to_string(),25,true);
//     // println!("{:?}",student_01);
//     student_01.get_student();    
    

//     let teacher_01 = mod_t::Teacher::new("Inzimam".to_string(),"Inzimam@yahoo.com".to_string(),35,true);
//     // println!("{:?}",teacher_01);
//     teacher_01.get_teacher();    
// }



// use student::{self,student_register,teacher_register,annoucements};
use student::*;
// use student::{self,teacher_register,student_register};
fn main(){
    let student_01 = student_register::Student::new("Faheem".to_string(),"Faheem@yahoo.com".to_string(),25,true);
    // println!("{:?}",student_01);
    student_01.get_student();    
    print_text();    

    let teacher_01 = teacher_register::Teacher::new("Inzimam".to_string(),"Inzimam@yahoo.com".to_string(),35,true);
    // println!("{:?}",teacher_01);
    teacher_01.get_teacher();    
}