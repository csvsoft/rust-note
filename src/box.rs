#[allow(dead_code)]
use std::mem;
use std::boxed::Box;

struct Point {x:i32,y:i32,}

fn make_point(x:i32,y:i32) ->Point {
     return Point{x:x,y:y}

}

fn make_boxed_point (a:i32, b:i32) -> Box<Point>{
   //let (a,b) = (x,y);
   let a_point = make_point(a,b);
   Box::new(a_point)
}

fn get_string() ->String{
    let str = String::from("hello");
    str
}

fn main(){
  let a_point_on_stack = make_point(1,2);
   println!("The size of point on stack is:{}",mem::size_of_val(&a_point_on_stack));

  let a_boxed_point = make_boxed_point(3,4);
    println!("The size of point on heap is:{}",mem::size_of_val(&a_boxed_point));

    let s = get_string();
    println!("I got a string:{}",s);



}