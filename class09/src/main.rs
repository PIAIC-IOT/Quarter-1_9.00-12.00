// #[derive(Debug)]
// enum EngineCC {
//     eight_hundred,
//     ten_hundred,
//     fifteen_hundred,
// }

// fn main() {
//     let boss = EngineCC::eight_hundred;    
//     let dragonR = EngineCC::ten_hundred;    
//     let SR = EngineCC::fifteen_hundred;
//     match dragonR {
//         EngineCC::eight_hundred => print!("Mehran"),
//         EngineCC::ten_hundred => print!("DragonR"),
//         EngineCC::fifteen_hundred => print!("Civic Turbo"),        
//     }
// }


// #[derive(Debug)]
// enum EngineCC {
//     eight_hundred,
//     ten_hundred,
//     fifteen_hundred,
// }
// #[derive(Debug)]
// struct Vehicle {
//     Name:String,
//     Color: String,
//     EngineCapacity: EngineCC,
// }
// impl Vehicle {
//     fn vehicle_info(&self){
//         println!("Vehicle Name:{}\nVehicle Color:{}\nVehicle CC:{}",
//             self.Name,
//             self.Color,
//             match self.EngineCapacity{
//                 EngineCC::eight_hundred => "800 CC",
//                 EngineCC::ten_hundred => "1000 CC",
//                 EngineCC::fifteen_hundred => "1500 CC",
//             }
//         );
//     }
// }
// fn main() {
//     let vehicle_01 = Vehicle{Name: "Civic".to_string(),
//         Color:"Silver".to_string(),
//         EngineCapacity: EngineCC::fifteen_hundred};    
//     vehicle_01.vehicle_info();
//     let vehicle_02 = Vehicle{Name: "Boss".to_string(),
//         Color:"Silver".to_string(),
//         EngineCapacity: EngineCC::eight_hundred};    
//     vehicle_02.vehicle_info();
// }


// #[derive(Debug)]
// enum Shape {
//     Rectangle{x:f64,y:f64},
//     Circle(f64),
//     Square(f64),
// }

// impl Shape {
//     fn area_sq(shape:Shape)->f64{
//         match shape {
//             Shape::Rectangle{x,y} => x*x as f64,
//             Shape::Circle(x) => x*x as f64,
//             Shape::Square(x) => x*x as f64,            

//         }
//     }
//     fn area_rect(shape:Shape)->f64{
//         match shape {
//             Shape::Rectangle{x,y} => x*x as f64,
//             Shape::Circle(x) => 0.0 as f64,
//             Shape::Square(x) => 0.0 as f64,            

//         }
//     }
//     fn area_cir(shape:Shape)->f64{
//         match shape {
//             Shape::Rectangle{x,y} => 0.0 as f64,
//             Shape::Circle(x) => 3.14* x*x as f64,
//             Shape::Square(x) => 0.0 as f64,            

//         }
//     }
// }

// fn main() {
//     let objRect = Shape::Rectangle{x:23.0,y:32.0};
//     let objCir = Shape::Circle(24.0);    
//         match objCir {
//             Shape::Rectangle{x,y} => println!("Area of Rectangle {}",x*y),
//             Shape::Circle(x)=> println!("Area of Circle {}",3.14*x*x),
//             Shape::Square(x)=> println!("Area of Square{}",x*x),
//         };
//     println!("The Area of Shpae Circle Is {}",Shape::area_cir(objCir));
//     println!("The Area of Shpae Rectangle Is {}",Shape::area_rect(objRect));
// }


// #[derive(Debug)]
// enum Shape {
//     Rectangle{x:f64,y:f64},
//     Circle(f64),
//     Square(f64),
// }

// impl Shape {
//     fn area(shape:Shape)->f64{
//         match shape {
//             Shape::Rectangle{x,y} => x*y as f64,
//             Shape::Circle(x) => 3.14*x*x as f64,
//             Shape::Square(x) => x*x as f64,            
//         }
//     }    
// }

// fn main() {
//     let objRect = Shape::Rectangle{x:23.0,y:32.0};
//     let objCir = Shape::Circle(24.0);            
//     let objSqu = Shape::Square(20.0);            
//     println!("The Area of Shpae Circle Is {}",Shape::area(objCir));
//     println!("The Area of Shpae Rectangle Is {}",Shape::area(objRect));
//     println!("The Area of Shpae Square Is {}",Shape::area(objSqu));
// }

fn Divide(x:f64,y:f64)->Option<f64>{
    if y == 0.0{
       None 
    }
    else{
        Some(x/y)
    }
}   

fn main(){
    println!("{:?}",Divide(24.0, 0.0).unwrap());
    match Divide(24.0, 0.0){
        Some(x) => println!("The Result Is This: {}",x),
        None => println!("Invlaid Divider"),
    };    
    println!("{}",
        match Divide(24.0, 0.0){
            Some(x) => format!("The Result Is This: {}",x),
            None => format!("Invlaid Divider"),
        }    
    );
}