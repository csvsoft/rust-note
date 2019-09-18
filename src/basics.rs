extern crate unicode_segmentation;

use unicode_segmentation::UnicodeSegmentation;


#![allow(dead_code)]
/**
* Reverse a tuple
*/
fn reverse_tuple(t:(i32,bool)) -> (bool,i32){
    let (a,b) = t;
    (b,a)
}

/**
*
*  Enums
* enums with implicit discriminators, starts with 0
**/
enum WebEvent{
    PageLoaded,
    PageUnloaded,
    KeyPress(char),  // tuple construct
    Point{x:i32,y:i32}  // structure construct

}

// enum with explicit discriminator
enum Color {
    Red = 0xff0000,
    Green = 0x00ff00,
    Blue = 0x0000ff,
}

fn inspect_web_event(e: &WebEvent){

    // use to import its variants into to scope
    use WebEvent::*;
    match e {
        _p@ PageLoaded => println!("Page loaded" ),
        PageUnloaded => println!("Page unloaded"),
        KeyPress(x) => println!("Key pressed:'{}'",x),
        WebEvent:: Point{x,y} => println!("Pointed at: ({},{})",x,y),
    }
}


/**Loop with 'label
*/
fn loop_test(){
    let mut i = 0;
    'aloop:loop{
        if i>10 {
            break 'aloop;
        }
        println!("{}",i);
        i = i + 1;
    }

    // return values from loop
    let mut counter = 1;
    let result = loop {
      if counter == 10 {
          break counter*2;
      }
        counter = counter + 1;
    };
    assert_eq!(result,20);


    // while loop
    let mut n = 1;
    while n < 10 {

        n = n + 1;
    }
}

fn casting(){
    let a_dec = 12.345_f32;
    let a_num = a_dec as i32;
    println!("decimal casted as a i32:{}",a_num);
}

use List::*;
enum List {
    Con(i32,Box<List>),
    Nil
}
impl List{
    fn new() -> List { Nil}
    fn prepend(self,e:i32) -> List{
        List::Con(e,Box::new(self))
    }
    fn len(&self) -> u32 {
        match self {
            Nil=> 0,
            Con(_, ref t) => 1 + t.len(),
        }
    }
}


fn swap(a:&mut i32,b:&mut i32){
    let t = *b;
    *b = *a;
    *a = t;
}

fn reverse(s:&str) -> String{

    UnicodeSegmentation::graphemes(s,true).rev().collect()

    //s.graphemes(true).rev().collect()

}




fn main(){

    // const
    const THRESHOLD: i32 = 10;

    let a_tuple = (10i32,true);
    let reversed_tuple = reverse_tuple(a_tuple);
    println!("Reversed tuple:{:?}",reversed_tuple);

    // enum
    let page_loaded = WebEvent::PageLoaded;
    inspect_web_event(&page_loaded);
    inspect_web_event(&WebEvent::Point {x:12,y:30});
    inspect_web_event(&page_loaded);
    // enum can be cast to i32
    println!("Red is: {:06X}",Color::Red as i32);


    //list
    let  list = List::new();
    let list = list.prepend(1);
    let list = list.prepend(2);
    println!("list size is:{}",list.len());

    // swap
    let mut a = 1;
    let mut b = 2;
    swap(&mut a,&mut b);
    println!("swapped:{},{}",a,b);

    // casting
    casting();

    // expression
    let x = 1;
    let z = {x*2};

    // loop
    loop_test();




}