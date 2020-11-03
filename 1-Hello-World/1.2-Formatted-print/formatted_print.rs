fn main() {
    println!("{} days", 31);
    println!("{0}, this is {1}. {1}, this is {0}", "Hailey", "Dillan");
    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond", "James");

    // Create a structure named `Structure` which contains an `i32`.
    #[allow(dead_code)]
    struct Structure(i32);

    let pi = 3.141592;

    // Pi is roughly: {arg 0 ("3.141592")} with precision specified inline (3)}
    println!("Pi is roughly: {:.3}", pi);

    // Pi is roughly: {arg 1 ("3.141")} with precision specified in arg 0 (5)}
    println!("Pi is roughly: {1:.0$}", 3, pi);

    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    // println!("This struct `{}` won't print...", Structure(3));
}