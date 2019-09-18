use std::collections::HashMap;

pub fn vector(){

    // create a new vector
    let  mut vectInt = vec![1,2,3];

    // update
    vectInt.push(4);
    println!("{:?}",vectInt);

    // iterating
    for x in &vectInt{
        println!("{}",x);
    }

    // read a value,[], or get(index) returns an Option<&T>
    let first = &vectInt[0];
    println!("First element of vector is:{}",first);

    match vectInt.get(5) {
        Some(x) => println!("Got a number:{}",x),
        None => println!("Out of index"),
    }


}

pub fn string(){
    // Creating
    // 1. from a string literal
    let str_a = "stra".to_string();
    // 2. String::from
    let str_b = String::from("strb");

    // Updating
    let mut str_c = "strc".to_string();
    str_c.push_str("extra");
    println!("expected strcextra, actual:{}",str_c);

    // concatenation
    let s1 = "s1";
    let s2 = String::from("s2");
    let s1s2 = format!("{}{}",s1,s2);
    println!("concatenated:{}",s1s2);
    let s3 = s1.to_string() + &s2;
    // s1 is not valid any more from this point, s3 takes the ownership of s1
    println!("s3:{}",s3);


}

pub fn hashmap(){
    use std::collections::HashMap;

    // Creating from constructor;

    let mut map1 = HashMap::new();

    // Creating from collection
    let teams = vec!["yellow","blue"];
    let scores = vec![10,20];
    let team_score :HashMap<_,_>= teams.iter().zip(scores.iter()).collect();

    map1.insert(String::from("key1"),String::from("value1"));
    map1.insert(String::from("key2"),String::from("value2"));
    map1.entry(String::from("key3")).or_insert("value3".to_string());

    match map1.get("key1"){
        Some(x) => eprintln!("I got {}",x),
        None => println!("I got nothing"),

    }


}

pub fn list_excercise(mut list: Vec<i32>){
    // get the average

    list.sort();
    let median =  list[list.len()/2];
    let mut sum:i32 = 0;
    let mut map:HashMap<i32,i32> = HashMap::new();

    for x in list.iter(){
        sum += sum + x;
        let value = map.entry(x.clone()).or_insert(0);


    }
      let avg = sum/(list.len() as i32);

    // get mod
    // let vec_count  = Vec::from_iter(map.iter());


}

pub fn pig_latin(str:&str) ->String {
    let mut new_str:String = String::from(&str[1..str.len()]);

    let c = str.chars().next().unwrap();
    if c == 'a' || c == 'e' || c=='o' || c == 'u' || c == 'i' {
        new_str = format!("{}{}",&new_str[..],"-jay");
    }else{
        new_str = format!("{}{}",&new_str[..],"-fay");
    }
    return new_str;

}
