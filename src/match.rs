
#![allow(dead_code)]
fn reference_match(){
    let a_i32_ref = &4i32;

   // a_i32_ref is a reference, &val is the type of reference.
    match a_i32_ref {
        &val => {
            println!("found {}",val);
            assert_eq!(val,4);
        },
    }

    // match the content of the reference, dereference it before the match
    match *a_i32_ref{
        val => assert_eq!(val,4)
    }

    let a_value = 4;
    match a_value {
        ref x if *x == 4 =>  assert_eq!(*x,4) ,// it created a reference 
        _ => println!("Unexpected match...") ,
    }

    // match a paire
    let pair = (1,2);
    match pair {
        (x,y) if x == y => println!("Found a twin"),
        (x,y) => println!("Found a regular pair:{},{}",x,y)
    }


}

fn if_let(){
    let someInt = Some(10);
    let noInt:Option<i32> = None;
    if let Some(i) = someInt {
        println!("I got a number:{}",i);
    }
    if let Some(x) = noInt {
        
    }else{
        println!("I got nothing");
    }
}

fn while_let(){
    let mut x = Some(10);
    while let Some(i) = x {
        if i<0{
            x = None;
        }else{
            x = Some(i-1);
        }
    }
    println!("I comleted while let");
}

fn match_binding(){
    let age = 32;
    match age {
        1...4 => println!("Toddlers."),
        5...14 => println!("teenagages"),
        a@14...18 => println!("age is:{}",a),
        x => println!("Not interested age:{}",x),
    }
}

fn main(){
 reference_match();
 match_binding();
 if_let();
 while_let();
}