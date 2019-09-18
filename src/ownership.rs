#![allow(dead_code)]
fn ownership_test(){

    {
    let _a = Box::new(5i32);
    // Now variable a owns the object allocated on the heap
    }
    // a has been destroyed here, since a exits its scope

    // The following statement will throw compilation errors
   // let b = a;

    for i in 0u32..1_0{
        let _x = Box::new(3i32);
        println!("{}",i)
    }

}

fn ownership_transfer(){
    let x = Box::new(1u32);

    // ownership is transferred from x to y, x is not valid variable now,
    let _y = x;

    let z = String::from("hello");
    // ownership is moved to the function
    foo(z);

    // the following statement is not valid
    //let k = z;

    let immutable_box = Box::new(1u32);
    let mut mutable_box = immutable_box;
    *mutable_box = 3;
    println!("now the mutable box value is:{}",*mutable_box);

}

fn borrow_str(s:&str){
    println!("I borrowed a str:{}",s);
}

fn foo(str:String){
    println!("I got a string and I consume it,{}",str);
}

struct Point{x:i32,y:i32}

struct MyObject;

impl Drop for MyObject{
    fn drop(&mut self){
        println!("My object dropped");
    }
}

fn re_bind(){
    let mut _a = MyObject;
    _a=MyObject;
    _a=MyObject;
}
fn main(){
    ownership_test();
    let _my_obj = MyObject;
    println!("Made a ToDrop!");
    ownership_transfer();

    let _a_str = String::from("Just fun.");
    let mut _a_mut_u32 = Box::new(1u32);

    //
    // At any time, when there is only ONE mutable borrower || multiple immutable borrowers.
    //let  a_mutable_u32_ref = &mut a_mut_u32;

    let ref _a_immutable_u32_ref = &_a_mut_u32;
    //let ref mut a_mutable_u32_ref = &mut a_mut_u32;

    //*a_mutable_u32_ref = 3;
    borrow_str(&_a_str);

    let mut _a = Box::new(1i32);
    _a = Box::new(1i32);

    println!("Before rebinding");
    re_bind();
    println!("After rebinding");



    //
    let mut p1 = Point{x:2,y:3};
    {
    let  p2 = &mut p1;
    p2.x = 4;
    }
    println!("Muted p1:({},{})",p1.x,p1.y);



}
