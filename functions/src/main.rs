fn main() {
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
