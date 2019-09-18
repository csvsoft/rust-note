### Package
A package a build unit with the following rules: cargo.toml + (0..1) library crate + (0..n) binary crate 
 * a cargo.toml file to describe how to build 1+ crates;
 * at least on crate;
 * 0..1 library crate;
 * n+ binary crate(s)
 
 
 
 Package Conventions
 * src/main.rs by default will create a binary crate with the name same as package
 * src/lib.rs create a library crate with the name of package

### Crate 
 A crate is a module tree
  * a crate root describe how to build the crate,it create the root module "crate"
 
 ### Module
 Module is the privacy boundary in RUST 
 
 module definition
 ```$rust
mod module_a{
  pub fn function_1(){
  println!("called module_a::function1");
}

```

if declare only  module name, rust will try to find a file named module.rs to load the module body.

```$java
mod module_b;

```