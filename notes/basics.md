
## Data Structures

### Struct
```$rust
// named field struct
struct Point {
 x: i32,
 y:i32,
}
// numeric struct
struct Message(i32,&str);
// zero sized struct
struct S;

```

## Enum
Enum is used to define algebraic data type :sum
Enum variant could be any data type: string, number, struct or even enum 
```bash
enum Message {
 Quit,
 Move { x:i32, y:i32},
 Write(String),
 ChangeColor(i32,i32,i32),
}
```
### Casting
Rust provides NO implicit type conversion (coercion) between primitive types. But, explicit type conversion (casting) can be performed using the as keyword.
```
let 

```

###Tuples
A tuple is a collection of values of different types. Tuples are constructed using parentheses (), and each tuple itself is a value with type signature (T1, T2, ...), where T1, T2 are the types of its members
```
// Declaration and initialization
let a = (1i32,true); 

// Tuple signature: (T1,T2,...Tn)
fn reverse_tuple(t:(i32,bool)) -> (bool,i32){
  let (a,b) = t;
  (b,a)
}

```

###Enum
Enum is a type which may be one of a few different variants
```
#![allow(dead_code)] 

```
###Box
All values in Rust are stack allocated by default. 
Values can be boxed (allocated in the heap) by creating a Box<T>.
A box is a smart pointer to a heap allocated value of type T. 

