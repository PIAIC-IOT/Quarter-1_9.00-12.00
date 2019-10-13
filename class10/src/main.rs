// fn main() {
    // let some_u8_value = Some(2);
    // match some_u8_value {
    //     Some(3) => println!("three"),
    //     Some(2) => println!("two"),
    //     _ => (),
    // }    

    // let some_u8_value = Some(3);
    // if let Some(3) = some_u8_value {
    //     println!("three");
    // }

//     let x = 3;
//     let y = 2;
//     if y == x{
//         println!("Equal");
//     } 
// }
 

// fn Area_Rect(l:i32,w:i32)->Option<i32>{
//     if l==0 || w == 0 {
//         None
//     }
//     else{
//         Some(l*w)
//     }    
// } 

// fn main(){
//     let area_01 = Area_Rect(0,4);
//     match area_01 {
//         Some(x) => println!("The Area of Rectangle Is:{}",x),
//         None => print!("You Enter the Zeroo"),
//     };
// }

extern crate reqwest;
fn main(){
    match reqwest::get("http://localhost:8080/api/") {
        Ok(mut response)=>{            
            match response.text() {
                Ok(text)=> println!("{}",text),
                Err(_) => println!("Error in Text"),
            }                    
        }
        Err(_)=> println!("Server Down")
    }    
}



