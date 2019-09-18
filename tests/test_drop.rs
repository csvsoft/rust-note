#[cfg(test)]

struct MyPointer{
    data:String,
}
impl Drop for MyPointer{
    fn drop(&mut self) {
        println!("My Pointer get dropped:{}",self.data)
    }
}

#[test]
fn test_drop(){
    let p = MyPointer{data:"Hello".to_string()};
    drop(p);  // drop the variable earlier

}

// Rust compiler will insert the code to run Drop.drop(..) to clean up the variable resources for variables in the scope that still have the ownership.