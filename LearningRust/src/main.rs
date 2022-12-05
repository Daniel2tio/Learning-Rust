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

#[test] //declares the function as a test function which is skipped during normal compilations
//can be seen as an example of an attribute,  its used to control compiler warnings and code style checks
//is included if run with "cargo test " command
fn test_gcd() { //defined function for testing
    assert_eq! (gcd(14,15), 1); //call the gcd function

    assert_eq! (gcd(2*3*5*11*17, 3*7*11*13*19),3*11);
}
