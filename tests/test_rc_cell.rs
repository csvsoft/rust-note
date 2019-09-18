use std::cell::RefCell;
#[cfg(test)]

trait Messenger {
    fn send(&self,msg:&str);
}

struct MockMessager{
    sent_messages:RefCell<Vec<String>>,
}

impl Messenger  for MockMessager{
    fn send(&self,msg:&str){
        &self.sent_messages.borrow_mut().push(msg.to_string());
    }
}

#[test]
fn test_rc_cell(){

}

