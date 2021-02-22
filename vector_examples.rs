// Vectors

use std::env;
use std::process::exit;

fn usage(msg: String){
    if !msg.is_empty(){
        println!("{}", msg);
    }
    println!("Usage: ./vector_examples N\n\nwhere N is number of items in the initial vector");
}

fn main(){

    let args: Vec<String> = env::args().collect();

    if args.len() != 2{
        usage(format!("Incorrect number of arguments, {} given, 2 required: {:#?}", args.len(), args));
        exit(1);
    } 
    
    /*
        Lots going on here.
        This is saying:
        Assign items as a usize from the result of the _match_ of the parse of args[1]
        So we parse and immediately match on it
        Which is passed in as `n`
        I've added logic because we can't have zero size vector, so handle that
        If it's zero, usage and exit(1), otherwise implicit return `n`
        If it's not a valid parse, most likely because of an invalid conversion
        from string to usize, then usage and exit(1)

        If all goes well, `items` has a non-zero, valid usize argument from the command line

    */
    let items: usize = match args[1].parse(){
        Ok(n) => {
            if n == 0{
                usage(format!("Number of items must be greater than 0, given: {}", args[1]));
                exit(1);
            }
            n
        },
        Err(n) => {
            usage(format!("Non-numeric item count given, retry with a number: {}", n));
            exit(1);
        }
    };

    // Vector
    let mut myvec = vec![ 0; items ];
    //let mut myvec = vec![1i32, 2, 3];
    println!("Vector: {:?}", myvec);

    myvec.push(4i32);
    myvec.push(5i32);

    println!("Vector: {:?}", myvec);
    
    myvec.pop(); // pops from end, [1,2,3,4]
    let x = myvec.pop(); // this is 4

    println!("Popped number should be 4: {:?}", x);
    
    let y = myvec.remove(2); // returns 3
    println!("Popped number should be 3: {:?}", y);
    
    println!("Vector: {:?}", myvec);

    let mut another_vec = vec![10i32, 11, 12, 13, 14];
    println!("Another Vector: {:?}", another_vec);

    myvec.append(&mut another_vec);
    println!("Vector with appended data: {:?}", myvec);
    println!("Another Vector: {:?}", another_vec);

    // Iteration
    for x in 0..myvec.len(){
        println!("First Form: Item: {:}", x);
    }

    // More Iteration
    let mut idx = 0;
    while idx < myvec.len(){
        println!("Second Form: Item: {:}", myvec[idx]);
        idx += 1;
    }

    // Even More Iteration
    for x in &myvec{
        println!("Third Form: Item: {:}", x);
    }

    // Slices


}
