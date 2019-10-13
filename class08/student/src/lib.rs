pub fn print_text(){
    println!("Hello WOrld");
}
pub mod annoucements {
    pub fn annoucements_students(){
        println!("Aj Chutti Nhi Hoeee UAn UAn Uaaoo");
    }
    pub fn annoucements_teacher(){
        println!("Aj Class Hogee");
    }
}

pub mod student_register{
    #[derive(Debug)]
    pub struct Student {
        name:String,
        email:String,
        age: i8,
        single: bool,
    }

    impl Student{
        pub fn new(name:String, email:String, age:i8, single:bool) ->Student
        {
            Student{
                name,
                email,
                age,
                single,
            }
        }
        pub fn get_student(&self){
            println!("Name:{}\nEmail:{}\nAge:{}\nSingle:{}",self.name,self.email,self.age,self.single);
            super::annoucements::annoucements_students();
        }
    }
}

pub mod teacher_register{
    #[derive(Debug)]
    pub struct Teacher {
        name:String,
        email:String,
        age: i8,
        single: bool,
    }

    impl Teacher{
        pub fn new(name:String, email:String, age:i8, single:bool) ->Teacher
        {
            Teacher{
                name,
                email,
                age,
                single,
            }
        }
        pub fn get_teacher(&self){
            println!("Name:{}\nEmail:{}\nAge:{}\nSingle:{}\n",self.name,self.email,self.age,self.single);
            super::annoucements::annoucements_teacher();
        }
    }
}

