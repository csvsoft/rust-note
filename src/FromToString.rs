use std::string::ToString;

struct Circle{ radius:u32}

impl ToString for Circle{
    fn to_string(&self) -> String{
        format!("circle of radius:{}",self.radius)
    }
}



fn main(){
    let c = Circle{radius:3};
    println!("circule:{}",c.to_string());



}