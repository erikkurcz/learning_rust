// Structs

// some basics
// tuple struct, effectively a named tuple
#[derive(Debug)]
struct Pair(i32, f32);

// struct with actual fields, similar to a C struct
#[derive(Debug)]
struct Point {
    x: i8,
    y: i8
}

// composition
#[derive(Debug)]
struct Rectangle<'a> {
    bottom_left: &'a Point,
    bottom_right: &'a Point,
    top_left: &'a Point,
    top_right: &'a Point,
}


fn main(){

    let corner = Pair(3200, 3.2);
    println!("{:#?}", corner);
    
    let a = Point{x:1,y:1};
    let b = Point{x:4,y:1};
    let c = Point{x:1,y:9};
    let d = Point{x:4,y:9};
    let container = [&a,&b,&c,&d];
    
    let rect = Rectangle{
        bottom_left: &a,
        bottom_right: &b,
        top_left: &c,
        top_right: &d
    };

    let mut index = 0;

    while index < 4 {
        println!("Point is: ({}, {})", container[index].x, container[index].y);
        index += 1;
    }
    
    println!("Construction of rectangle: {:#?}", rect);

}
