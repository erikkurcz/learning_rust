// Demonstrate debug printing of structures

// cannot be printed using fmt::Display or fmt::Debug
struct Unprintable(i32);

// has derived structures internally to print debug
// still cannot be printed regularly
#[derive(Debug)]
struct DebugPrintable(i32);


// some pretty printing
#[derive(Debug)]
struct Person<'a> {
    name: &'a str,
    age: u8
}

fn main(){
    // named '_x' to let compiler now this is intentionally unused code
    let _x = Unprintable(15);
    let y = DebugPrintable(1+26);
    
    //println!("{x}", x=x); // doesn't compile, not printable with std::fmt
    println!("{:?}", y);
    //println!("{0}", y); // also doesnot compile because not printable with std::fmt, even if it can be with debug on previous line

    // pretty print example
    let name = "Peter";
    let age = 27;
    let peter = Person { name , age };

    println!("{:#?}", peter)


}

