mod sound;
mod collection;
mod traits;
fn main() {
    println!("Hello, world!");
    sound::instrument::clarinet();
    collection::vector();
    collection::string();
    collection::hashmap();
   let pig = collection::pig_latin("hello");
    assert_eq!("ello-fay".to_string(),pig);

}
