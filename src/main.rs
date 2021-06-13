pub mod m;

fn for_each_planet<F>(f: F)
    where F: Fn(&'static str)
{
    f("Earth");
    f("Mars");
    f("Jupiter");
    let x = String::from("kk");
    f(&x)
}
 
fn main() {
    for_each_planet(|planet| println!("Hello, {}", planet));
}

// prints:
// Hello, Earth
// Hello, Mars
// Hello, Jupiter
/*

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}

fn main() {

    let x = 5;
    let mut y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);
    *y += 1;
    assert_eq!(5, x);
    assert_eq!(6, *y);

    
    let mut a = 5;
    assert_eq!(5, a);

    let b = a;
    assert_eq!(5, b);
    
    a = 3;
    assert_eq!(3, a);
    assert_eq!(5, b);
    takeArg(a);
    assert_eq!(3, a);
    
    let s = String::from("hello");
    tstr(&s);
    println!("{:#?}", s);
}
fn tstr(s: &String) {
    println!("{:#?}", s);
    
}

fn takeArg(arg: u8) {
    println!("{:#?}", arg);
}
*/
