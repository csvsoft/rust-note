#![allow(dead_code)]

//use std::box::Box;

const MAX_PLAYERS:u32 = 10;
struct s_named{
    x:u32,
    y:u32,

}


struct s_numbered(i32);

impl Copy for s_numbered{

}

impl Clone for s_numbered {
    fn clone(&self) -> s_numbered {
        *self
    }
}

struct s_unit;

/*
Enum variants could be different data types: String,number,or struct
*/
enum IPAddress{
    V4,V6
}

enum Message {
    Quit,
    Move { x:i32, y:i32},
    Write(String),
    ChangeColor(i32,i32,i32)
}

fn test_ipaddress(){
    let v4 = IPAddress::V4;

}

fn basic_types(){

    let a:i64;
    a=54;
    println!("a={}",a);

    let b:Box<i32>;
    b = Box::new(30);
    println!("b = {}",*b);

    // Copy value of move value, Clone is the super trait of Copy
    let a = 2;
    let b = a; // copy a's bytes to b , if b is of Copy
    let c =a;
    assert_eq!(c,2);

    let struct_1= s_numbered(2);
    let struct_2 = struct_1; // value copied, if the source is Copy


    let struct_3 = struct_1;


    let struct_4 = s_named{x:1,y:2};
    let struct_5 = struct_4;  // value moved, s_named is NOT a Copy
    // compilation failed.
    //let struct_6 = struct_4;

    println!("struct_3:({})",struct_3.0);


    // tuple
    let tuple_1 = (1,2);
    let tuple_2:(i32) = (23);
    println!("tuple:{}",tuple_1.0);

    //array
    let array_1=[1];
    println!("array element:{}",array_1[0]);

    let array_2:[&str;2]=["hello","world"];
    println!("array_2:{}",array_2[1]);

    let array_3:[i32;500] = [22;500]; // [x;n] create an array with n copy of x


    // slice
    let slice_1 = &array_1;
    let slice_2 = &array_2[1..2]; //[startIndex..endIndex(exclusive];
    println!("slice_2:{}",slice_2[0]);

    // string slice
    let string_1 = String::from("You are beautiful");
    let str_slice = &string_1[0..2]; // string slice is a reference to a part of string
    let str_slice_full = &string_1; // full slice;
    let str_slice_2 = "You are greate"; // string literal is a string slice;
    println!("string slice:{}",str_slice);

    // range
   //TODO

}

fn build_struct(){

    let s_named1 = s_named{x:1,y:1};
    let s_numbered_1 = s_numbered(20);
    let s_unit_1 = s_unit;


}
fn main(){

    basic_types();
    build_struct();

}