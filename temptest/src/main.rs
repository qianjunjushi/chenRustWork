use std::borrow::Cow;

fn main() {
    let x=&3;
    let &y=x;
    
    println!("Hello, world!{}",y);
}
