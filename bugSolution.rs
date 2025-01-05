fn main() {
    let mut x = 5;
    { //creating a new scope so that the mutable borrow is released before the creation of the immutable borrow.
        let y = &mut x;
        *y = 10; 
    }
    let z = &x; 
    println!("x = {}", x);
    println!("z = {}", *z); 
}