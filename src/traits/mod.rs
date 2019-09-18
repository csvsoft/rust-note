use std::cmp::PartialOrd;

pub fn largest_num<T: PartialOrd + Copy>(list:&[T]) -> T{
    let mut largest: T = list[0];
    for &x in list.iter(){
        if x > largest{
            largest = x;
        }

    }
    largest

}