### Ownership
Scope plays an important role to indicate to compilers when
* Variables are created and destroyed;
* Resources are freed;
* Borrowers are valid.

An object can only be owned by ONE variable;

#### ownership transfer(move)
When 
* doing assignments (let x = y) or 
* passing function arguments by value (foo(x));

 the ownership of the resources is transferred, in Rust it is known as a move
 
 After the ownership gets transferred, the original owner variable is NOT VALID any more.
 The mutability could be changed when transferring the ownership;

### RAII
Resource Acquisition Is Initialization.
Variables in Rust not only hold the data allocated in the stack , they also OWNS the resources allocated on 
heap. 
When the variable goes out the scope , the resources it owns will be destroyed ***By calling its destructor***.

 
### Borrowing
Then variables don't want to take the ownership, they can borrow the object by reference
RUST compiler will guarantee the object it references is valid.

 At any time, when there is only ONE mutable borrower || multiple immutable borrowers.
 
 Borrowers can borrow immutable reference from a mutable object owner.

 When there is any references in the scope , the resource is frozen.