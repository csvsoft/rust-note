#[cfg(test)]
mod tests {
    #[test]
    fn test_1(){
        assert_eq!(2+2,4);
    }

    #[test]
    fn test_remove_vowels(){
        assert_eq!(remove_ovels(&"hello".to_string()),"hll");
    }
    fn remove_ovels(txt:&String) -> String{
        let mut ret = String::new();
        for c in txt.chars(){
            match c {
                'a' | 'e' | 'i' | 'o' | 'u' => continue,
                _ => ret.push(c),
            }
        }
        return ret;
    }
}
