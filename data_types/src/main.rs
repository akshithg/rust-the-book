fn main() {
    let int8: u8 = 255;
    println!("int8 = {}", int8);

    // with cargo build :: causes panic
    // with cargo build --release :: wraps around
    // let mut int8: u8 = 255;
    // int8 += 1;
    // println!("int8 = {}", int8);

    let x = 2.0; // f64
    let y:f32 = 3.0; // f32

    println!("x = {}, y = {}", x, y);

    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    // unicode characters

    println!("c = {} z = {} cat = {}", c, z, heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    // a tuple has type for each position

    //  destructuring a tuple
    let (x, y, z) = tup;
    println!("tup = (x,y,z) = ({}, {}, {})", x, y, z);

    // or use a period (.)
    let a = tup.0;
    let b = tup.1;
    let c = tup.2;
    println!("tup = (tup.0, tup.1, tup.2) = ({}, {}, {})", a, b, c);

    // array
    // use when size is fixed, else use a vector
    let days_of_the_week = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let _first_five_primes: [u32; 5] = [2, 5, 7, 11, 13];
    let _a = [10; 4]; // [10, 10, 10, 10]

    let _first_day =  days_of_the_week[0];
    let _second_day =  days_of_the_week[1];
    // index out of bound causes panic
    // let eighth_day = days_of_the_week[7];

}
