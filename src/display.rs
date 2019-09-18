use std::fmt;

struct Point {x:i32, y:i32 }

impl fmt::Display for Point{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
       return  write!(f,"({},{})",self.x,self.y)
    }
}

struct Color{ r:u8,g:u8,b:u8}

impl fmt::Display for Color{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return  write!(f,"0x{:02X}{:02X}{:02X}",self.r,self.g,self.b)
    }
}

struct List(Vec<i32>);

impl fmt::Display for List{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let vec = &self.0;
        write!(f,"[")?;
        for (count,v) in vec.iter().enumerate(){
            if count != 0 { write!(f,",")?;}
            write!(f,"{}:{}",count,v)?;
        }

        return  write!(f,"]");
    }
}



fn main(){
    let p = Point{x:1,y:2};
    println!("Point:{}",p);
    let c = Color{r:126,g:255,b:0};
    println!("Color:{}",c);

    let v = List(vec!(1,3,5));
    println!("list:{}",v)

}