use std::ops::Deref;

#[cfg(test)]


struct MyBox<T>(T);
impl<T> MyBox<T>{
    fn new(x:T) -> MyBox<T>{
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T>{
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn hello(name:&str) ->String {
    format!("Hello {}",name)
}

#[test]
fn test_deref(){

    let myBox = MyBox::new(String::from("world"));
    assert_eq!("Hello world",hello(&myBox));

}


#[test]
fn test_result_type(){
    use std::fs::File;

}

