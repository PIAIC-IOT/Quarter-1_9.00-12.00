// fn main() {
//     panic!("crash and burn");
// }

// export RUST_BACKTRACE=1


// use std::fs::File;
// fn main() {
//     let f = File::open("hello.txt");
// }


// use std::fs::File;
// fn main() {
//     let f = File::open("hello.txt");
//     let f = match f {
//         Ok(file) => println!("{:?}",file),
//         Err(error) => {
//             panic!("Problem opening the file: {:?}", error)
//         },
//     };    
// }

// use std::io;
// use std::io::Read;
// use std::fs::File;

// fn main(){
//    println!("{:?}",read_username_from_file().unwrap()); 
// }
// fn read_username_from_file() -> Result<String, io::Error> {
//     let mut f = File::open("hello.txt")?;
//     let mut s = String::new();
//     f.read_to_string(&mut s)?;
//     Ok(s)
// }


use std::fs;
fn main(){
    let data = "Some data!";
    fs::write("9-5.txt", data).expect("Unable to write file");
}

./mono open file.txt <