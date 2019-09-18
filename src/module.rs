#[allow(dead_code)]
mod my_mod{
    fn private_fun(){
        println!("Hello, this is a private function")
    }
    pub fn a_pub_fun(){
        println!("Hello, this a is public function")
    }

    pub struct Open_Box<T>{
        pub content:T,
    }

    mod nested_mod{
        fn private_fun(){
            println!("This is a function from nested_mode");
            super::private_fun()
        }
        fn another_fun(){
            self::private_fun();
        }
    }

    pub struct Close_Box<T>{
        content:T,
    }
    impl<T> Close_Box<T>{
        pub fn new(content:T) -> Close_Box<T> {
            Close_Box{
                content:content,
            }
        }
    }
}
use my_mod::a_pub_fun as my_mod_fun;  // use as to rname the member 
use mod_a;
fn main(){
    let open_box = my_mod::Open_Box{content:"helo"};
    println!("open box content:{}",open_box.content);

    let close_box = my_mod::Close_Box::new("close box");
   // println!("close box content:{}",close_box.content)
   my_mod_fun();
   mod_a::fun_2();



}