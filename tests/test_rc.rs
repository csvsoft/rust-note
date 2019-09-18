use std::rc::Rc;

#[cfg(test)]

enum List{
    Con(i32,Rc<List>),
    Nil,
}

#[test]


fn test_rc(){
    use crate::List::{Con,Nil};
    let a = Rc::new(Con(10, Rc::new(Con(5,Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Con(3,Rc::clone(&a));


    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Con(4 ,Rc::clone(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));


}