pub mod read_inputs{
    use std::io;
    pub fn read()-> String{
        let mut input = String::new();
        io::stdin().read_line(&mut input);
        input
    }        
}