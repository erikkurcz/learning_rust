// Basic hello world program


fn main(){
    
    // printing via macro
    println!("Hello world!");
    println!("I'm a Rustacean!");

    // String formatting
    let x = 5 + 5;
    println!("x is equal to 10: x = {}", x);

    // Different prints
    print!("no extraline");
    println!("with an extra line");

    eprint!("Error text, pipe to see");
    eprintln!("Error newline, pipe to see");
    
    // More string formatting
    println!("{} it up, count {} up, count it {}, COUNT IT!", "count", "it", "up");

    println!("The quick brown {fox}, jumped over the {lazy} dog", fox="bear", lazy="sleeping");

    // Suffixes
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);
    println!("My name is {0}, {1} {0}", "Bond", "James");
    
    // Number formatting
    let pi = 3.14159;
    println!("Pi is equal to: {1:.*}", 3, pi);


}
