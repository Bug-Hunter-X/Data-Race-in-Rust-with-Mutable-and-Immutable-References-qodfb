fn main() {
    let mut x = 5;
    let y = &mut x;
    *y = 10;
    let z = &x; 
    println!("x = {}", x);
    println!("z = {}", *z); 
}