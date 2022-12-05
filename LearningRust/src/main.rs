fn main() {
    println!("Hello, world!");
}


//We use fn to define a function which in this case is called "gcd"
//We assign two parameters to the function, n and m and assign a data type of unsigned 64bit integers
//We then set the return type of the function to be the same which is u64
//mut means that the variables can be mutable 
//Rust holds two floating types which are f32 and f64(float and double)
fn gcd (mut n:u64, mut m:u64) -> u64 {
    assert!(n != 0 && m != 0);  //assert checks that the argument is true (neither argument is zero)
    while m != 0 {
        if m < n {
            let t = m; //a let statement declares a local variable (t)
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}
