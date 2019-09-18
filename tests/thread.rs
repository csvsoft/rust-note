#[cfg(test)]

use std::thread;
use std::time::Duration;
use std::sync::mpsc;

#[test]
fn test_thread_1(){
    let t1 = thread::spawn(||{
        for i in 1..15{
            println!("Hello, number {} from spawned thread",i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    let t2 = thread::spawn(||{
        for i in 1..10{
            println!("Hello, number {} from spawned thread2",i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    t1.join().unwrap();
    t2.join().unwrap();

    let v = vec![1,2,3];
    let t3 = thread::spawn(move||{
        for x in v.iter(){
            println!("hi thread {}",x);
        }
    });

    for i in 1..20 {
        println!("Hi number {} from main thread.",i);
        //thread::sleep(Duration::from_millis(1));
    }
    t3.join();

}

#[test]
fn test_mutex(){
    use std::sync::{Mutex,Arc};
    use std::thread;
    let m = Mutex::new(5);
    {
        let mut n = m.lock().unwrap();
        *n = 6;
    }
    println!("Now value is:{:?}",m);

    let m2 = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for i in 0..10{
        let counter = Arc::clone(&m2);
        let handle = thread::spawn(move ||{
            let mut x = counter.lock().unwrap();
            *x += 1;
        });
        handles.push(handle);
    }

    for h in handles{
        h.join().unwrap();
    }
    println!("result is :{}",*m2.lock().unwrap());
    assert_eq!(10,*m2.lock().unwrap());
}

#[test]
fn test_channel(){

    let (tx,rx)= mpsc::channel();
    let tx2 = mpsc::Sender::clone(&tx);

    let thread_1 = thread::spawn(move ||{
       let strs = vec![String::from("str1"),String::from("str2"),String::from("str3")];
        for x in strs{
            tx.send(x);
            thread::sleep(Duration::from_millis(20));
        }
    });

    let thread_2 = thread::spawn(move ||{
        let strs = vec![String::from("str1_2"),String::from("str2_2"),String::from("str3_2")];
        for x in strs{
            tx2.send(x);
            thread::sleep(Duration::from_millis(20));
        }
    });

    for y in rx{
        println!("Received:{}",y)
    }

    thread_1.join();
    thread_2.join();

}

#[test]
fn reverse_str(str:&str) -> String{

}