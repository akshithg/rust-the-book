fn main() {
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
