#![allow(dead_code)]
fn closure_test(){
    fn plus_one(i:i32) -> i32 {i + 1}
    let closure = | i:i32| -> i32 {i+1};
    let closure2 = |i:i32| i + 1i32;
  
    println!("function call:{}",plus_one(1));
    println!("closure call:{}",closure(1));
    println!("closure2 call:{}",closure2(1));
    println!("closure call:{}",closure(1));
    
    
}

fn diverge_test(){

}

fn sum_odd_num(up_to:i32) -> i32{
    let mut sum:i32 = 0;
    for i in 0..up_to{
        let odd_num = match i%2 == 1 {
            true => i,
            false => continue,
        };
         sum +=  odd_num;
    }
    return sum;
}



fn main(){
    closure_test();
    let sum = sum_odd_num(4);
    println!("sum of odd numbers: {}",sum);
}