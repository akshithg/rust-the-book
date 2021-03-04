use std::io;

fn main() {
    variables();
    data_types();
    functions();
    control_flow();
}

fn variables() {
    let x = 5;
    println!(" x = {}", x);

    // x is immutable
    // x = 6;

    // make x muatable using mut
    let mut x = 5;
    println!(" x = {}", x);
    x = 6;
    println!(" x = {}", x);

    // Constants
    const MAX_SCORE: u32 = 99_999;
    println!("The maximum score possible is: {}", MAX_SCORE);

    // Shadowing
    let spaces = "  ";
    println!(" spaces = |{}|", spaces);

    // Shoadowing redefines value
    let spaces = "      ";
    println!(" spaces = |{}|", spaces);

    // Can change type too
    let spaces = spaces.len();
    println!(" spaces = {}", spaces);

    // Can not change type of mut variable
    // x = "Seven"
}

fn data_types() {
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

fn functions() {
    another_function(5, 6);

    // let x = 1 is a statement, no return
    // 3 + 6 is an expressions, returns
    // x = y = 5 not possible, cuz (y=5) doesn't return

    // stuff within {...} is an expression, x + 1 should NOT end with a
    // semicolon if it DOES, it IS a statement.
    let y = {
        let x = 9;
        x + 1
    };

    println!("The value of y is: {}", y);

    let a = 10;
    let b = double(a);
    println!("b = a * 2. a = {}, b = {}", a, b);

}

fn another_function(x: i32, y: i32) {
    println!("x = {}", x);
    println!("y = {}", y);
}

fn double(x: u32) -> u32 {
    x * 2
    // x * 2;
    // this is will not return with the semicolon at the end
}


fn control_flow() {
    let x = 5;

    if x < 10 {
        println!("x < 10");
    } else {
        println!("x >= 10");
    }

    // if cannot take non bool expressions
    // let x = 0;
    // if x {} ...

    let y = if x % 5 == 0 { 0 } else { 1 };

    println!("y = if x % 5 == 0  |0| else |1|, y = {}", y);

    // let y = if ... {} else {} should have same types
    // let y = if x % 2 == 0 { true } else { 1 }

    //  loops
    let mut count = 0;
    let result = loop {
        count += 1;
        if count == 10 {
            break count / 2
        }
    };
    println!("loop result  = {}", result);

    let mut count_down = 10;

    while count_down != 0 {
        println!(" {} ", count_down);

        count_down -= 1;
    }
    println!("🚀");

    let days_of_the_week = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    for day in  days_of_the_week.iter() {
        println!("{}", day)
    }

    for i in (1..10).rev() {
        println!(" {} ", i);
    }
    println!("🚀");

    println!("nth Fib: n ?");

    let mut n = String::new();

    loop {
        io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

        let n: u32 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        let mut a = 0;
        let mut b = 1;
        let mut c = a + b;
        let mut i = n;
        while i > 0 {
            c = a + b;
            a = b;
            b = c;
            i -= 1;
        }
        println!("{}th fib = {}", n, c);
        break;
    }
}
