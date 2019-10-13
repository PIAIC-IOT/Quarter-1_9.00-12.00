pub mod student_work{
    
    #[derive(Debug)]
    pub struct Student{
        pub first_name: String,
        pub last_name: String,
        pub age: i8,
        pub single: bool,
        pub height: f32, 
    }
    impl Student{
        pub fn student_information(&self) -> String{
            let student = format!("Student Name:{} {}\nStudent Age:{}\nStudent Height:{}",
            self.first_name,self.last_name,self.age,self.height);
            student        
        }    
    }   
}
// pub fn student_main(){
//     println!("Hello from student file");
// }