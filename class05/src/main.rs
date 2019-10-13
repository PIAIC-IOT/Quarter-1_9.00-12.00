use std::collections::HashMap;

fn main() {

    let mut arr = [3;5];
    arr[2] = 6;
    // arr[5] = 10;
    println!("{:?}", arr);

    let mut vector_01: Vec<i32> = Vec::new();//
    vector_01.push(10);
    vector_01.push(23);
    vector_01.push(34);
    vector_01.pop();

    let vector_02 = vec!["Faheem","Ibad","Zain"];

    let mut vector_03 = vec![12,45,76,978,234];
    vector_03[0] = 23;
    vector_03[1] = 43;
    vector_03[2] = 63;
    vector_03[3] = 43;

    
    // vector_01[10] = 43;

    println!("{:?}", vector_01);
    println!("{:?}", vector_02);
    println!("{:?}", vector_03);


    let v = vec![100, 32, 57];
    let mut sum = 0;
    for i in &v {
        sum = sum+i;
    }
    println!("The Sum is {}", sum);

    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i *= *i;
        println!("{}", i);
    }
    println!("{:?}", v);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    // let s_02 = format!("{} - {} - {}",&s1,&s2,&s3);
    // println!("{}",s_02);
    let len = String::from("Hola?З"); // ASCII it takes only one byte
    let len_02 = String::from("Здравствуйте"); // UTF-8 it takes only two byte

    for x in len_02.chars(){
        println!("{}", x);
    }
    println!("{}, {}",len,len_02);

    let mut vector_01 = vec![2, 25, 9];

    for index in 0..vector_01.len(){
        vector_01[index] = vector_01[index]*vector_01[index]*vector_01[index]; 
    }
    println!("the cude vector is: {:?}", vector_01);

    println!("{:?}","Здравствуйте".bytes());

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    println!("{:?}", scores);

    // let teams  = vec![String::from("Yellow"),String::from("Blue")];
    // let initial_scores = vec![50,10];

    // let teams = [String::from("Yellow"),String::from("Blue")];
    // let initial_scores = [50,10];

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let field_car = String::from("Favorite Car");
    let field_value_car = String::from("Corola");
    let field_update_car = String::from("Favorite Car");
    let field_value_update_car = String::from("Civic");

    
    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    map.insert(field_car, field_value_car);
    map.insert(field_update_car, field_value_update_car);
    println!("{:?}", map);
    
    let map_value = map.get(&"Favorite Car".to_string());
    println!("{}", map_value.unwrap());

    
    map.entry(String::from("Favorite Car")).or_insert(String::from("Civic"));

    for (key, value) in &map {
    println!("{}: {}", key, value);
    }


    let text = "hello world wonderful world world";

    let mut map = HashMap::new();
    println!("{:?}", text.split_whitespace());
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

