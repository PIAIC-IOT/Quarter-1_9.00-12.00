fn main() {
    // // Variable Declaration and Initialization 
    // let x = 10;    
    // println!("{:?}",x);
    // //Primitive Types
    // let x =10;
    // let y =10.2;
    // let z = true;
    // println!("{:?},{:?},{:?}",x,y,z);
    // //Compound Types
    // let sameData = [3,4,5,6,7];
    // println!("{:?}",sameData[1]);   

    // let diffData = (2,"Hello",true);
    // println!("{:?}",diffData.1);

    // // Conditions
    // let a = 10;
    // let b = 20;
    // let c = 8;
    // if a < b  // a < b => a,b are oprands and < is the operator
    // {
    //     println!("a is less then b");
    // }    
    // else {
    //     println!("a is greater then b");
    // }

    // if a < b && a < c   // a && b where AND is the logical operator {Logical Operators are: AND, OR, NOT}
    // {
    //     println!("a is less then b and c");
    // }
    // else if a < b && a > c{
    //     println!("a is less then b and greater then c");
    // }
    // else {
    //     println!("a is greater then b and c");
    // }
    // // Match Statment
        // let x = 10;
        // let y = 20;
        // match x {
        //     10 => println!("The Value is Ten"),
        //     _ => println!("The Value is Not Ten"),
        // }
        // let x = 10; // Shadowing 
        // match x<y {
        //     true => println!("x is less then y"),
        //     false => println!("x is greater then y"),
        //     _ => println!("The Value is Not valid"),
        // }
        // println!("{}",
        //     match x {
        //         10=> "The Value is Ten",
        //         _=> "The Value is not Ten",
        //     }
        // );        
        
        // // Iterative Condition
        // for x in 0..=10{
        //     println!("{}",x);
        // }
		
        // // linear sorting

        let mut sameData = [3,3,5,4,8,6];                
        let mut temp = 0;
        for x in 0..sameData.len(){                        
            for y in x..(sameData.len()-1){
                if(sameData[y+1]<sameData[x]){
                    temp = sameData[y+1];
                    sameData[y+1] = sameData[x];
                    sameData[x] = temp;
                }
            }
        }
        println!("{:?}",sameData);

}
