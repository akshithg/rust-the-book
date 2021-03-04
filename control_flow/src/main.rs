use std::io;

fn main() {
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
