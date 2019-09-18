fn un_recoverable_error(){
    panic!("Not able to proceed, non recoverable errors");
}

fn check_even(i:i32) -> Result<String,String>{
   return  match i%2 {
        0 => Ok("Is an even number"),
        1 => Err("It is a odd number"),
    }
}