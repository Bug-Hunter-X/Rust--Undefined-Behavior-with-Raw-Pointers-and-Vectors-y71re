fn main() {
    let mut v = vec![1, 2, 3];
    // Safe and preferred way to modify vector elements
    v[0] = 10; 
    println!("{:?}", v);
}