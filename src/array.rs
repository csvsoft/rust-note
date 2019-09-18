use std::mem;

fn create_array() -> [u32;5]{

    println!("Creating an array");
    // declare an array [T;size]
    let a_int_array :[u32;5] = [1,2,3,4,5];

    // Loop an array
    for i in a_int_array.iter(){
        println!("{:?}",i);
    }
    println!("size:{:?}",a_int_array.len());

    //
    println!("First element of the array is:{:?}",a_int_array[0]);
    println!("Bytes of the array:{:?}", mem::size_of_val(&a_int_array));
    return a_int_array;

}

fn test_slice(s:&[u32]){
    println!("slice size:{:?}",s.len());
    println!("First element of the slice is:{:?}",s[0]);


}
fn main(){

 let a_array = create_array();
    test_slice(&a_array[1..4]); // 1 until 4

}