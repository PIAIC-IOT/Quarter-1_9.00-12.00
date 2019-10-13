// #[derive(Debug)]
// struct Student{
//     first_name: String,
//     last_name: String,
//     age: i8,
//     single: bool,
// 	height: f32, 
// }

// fn main() {
//     let mut student_01 = Student{
//         single: true,
//         first_name: "Faheem".to_string(),
//         last_name: "Uz Zaman".to_string(),
//         age: 24,
//         height: 5.8,
//     };
//         let student_02 = Student{
//         single: true,
//         first_name: "Zain".to_string(),
//         last_name: String::from("Ul Abdin"),
//         age: 25,
//         height: 5.9,
//     };
//     student_01.first_name = "Fahim".to_string();
//     println!("{:?}", student_01);
//     println!("{}", student_01.first_name);
// }




// #[derive(Debug)]
// struct Student{
//     first_name: String,
//     last_name: String,
//     age: i8,
//     single: bool,
// 	height: f32, 
// }

// fn main() {
// let mut student_01 = create_student("Fahim".to_string(),"Uz Zaman".to_string(),24,true,5.8);
// student_01.first_name ="Faheem".to_string();
// println!("{:?}", student_01);
// }

// fn create_student(f_n:String,l_n:String,age:i8,s:bool,h:f32)-> Student{
//     Student{
//         first_name: f_n,
//         last_name: l_n,
//         age: age,
//         single: s,
//         height: h,
//     }
// }


#[derive(Debug)]
struct Student{
    first_name: String,
    last_name: String,
    age: i8,
    single: bool,
	height: f32, 
}

fn main() {
let mut student_01 = create_student("Fahim".to_string(),"Uz Zaman".to_string(),24,true,5.8);
student_01.first_name ="Faheem".to_string();
println!("{:?}", student_01);
let car1 = Car{
    Name: "FX".to_string(),
    Color: "White".to_string(),
    CC: "234f324rf24r34".to_string(),
    Model: 1978,
    Type: "Hatchback".to_string(),
};
println!("{:?}", car1);


}

fn create_student(first_name:String,last_name:String,age:i8,single:bool,height:f32)-> Student{
    Student{
        first_name,
        last_name,
        single,
        height,
        age,
    }
}


// Write a program in which you make a struct for 
// Car=> Name, Color, CC, Model, Type,
// 
#[derive(Debug)]
struct Car{
    Name:String,
    Color: String,
    CC: String,
    Model: i32,
    Type: String,
}

