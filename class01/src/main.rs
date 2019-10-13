fn main() {
    
    // variable and mutability 
    let mut my_name_friend = "Ibad"; // varibale declaration and varibale binding 
    my_name_friend = "Zain";
    println!("{}",my_name_friend);
    
    // constant
    const PI: f32 = 3.14; // constant declaration and constant binding 
    println!("Constant Is: {}",PI);

    //scope of variable
    let my_fname = "Faheem"; 
    {
        let my_friend_name = "Zain"; // scope limited to this block only
        println!("{}",my_fname);
    }
    
    //shadowing 
    let my_name = "faheem01";
    println!("{}",my_name);
    let my_name = my_name.len(); // Performing Shadowing
    println!("{}",my_name);  

    // Primitive Types (Scaler Types)
    let mut char_var = '😎'; 	// character type
    let int_var = 20;		// integer type
    let float_var = 30.2;	// floating point type
    let bool_var = true; 	// boolean type (treue / false)
    println!("Char: {} Integer: {} Float: {} Boolean: {}",char_var,int_var,float_var,bool_var);// "{}" this refers to place holders

    let char_var = 'B'; //Shadowing also use with same data type and different but you must use the (let <same_variable_name>) these both things
    println!("{}",char_var);
    let char_var = true; //Shadowing also use with same data type and different but you must use the (let <same_variable_name>) these both things
    println!("{}",char_var);


   // Data types
   let age:u32 = 20;
   let sum:i32 = 5-15;
   println!("sum is {} and age is {}",sum,age);

   // 0 to 255 only allowed for u8
   let weight:u8 = 255;
   println!("weight is {}",weight);

   let interest:f32 = 8.35;
   let cost:f64 = 15000.600;  //double precision
   
   println!("interest is {}",interest);
   println!("cost is {}",cost);

   let float_with_separator = 11_000.555_001;
   println!("float value {}",float_with_separator);
   
   let int_with_separator = 50_000;
   println!("int value {}",int_with_separator);

   let isfun:bool = true;
   println!("Is Rust Programming Fun ? {}",isfun);

   let emoji:char = '😁';   
   println!("emoji is {}",emoji);
}
