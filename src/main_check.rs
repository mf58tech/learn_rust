fn main() {
    let mut s = String::from("hello");
    tstr(s);
    println!("{:#?}", s);
/*    
    //what if using a u8? 
    //scalar has no ownership, it's always copy?

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{} and {}", r1, r2);
    // r1 and r2 are no longer used after this point

    //let r3 = &mut s; // no problem
    //new thread  thread_use(&mut s)
    //now change s
    //todo  this leads to 2 pointers modifying s?
    s.push_str("jjj")
    println!("{}", r3);
    */
}
fn tstr(s) {
    println!("{:#?}", s);
    
}

fn thread_use (st: &mut String ) {
    //just read it
    //run me in a new thread
    //sleep(5)
    st.push_str("kkk")
}

