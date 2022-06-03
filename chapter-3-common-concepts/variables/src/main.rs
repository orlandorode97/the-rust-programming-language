const MAX_POINTS: u32 = 100_000;
fn main() {
    let mut var = 5;
    println!("This is the value of var {}", var);
    var = 6;
    println!("This is the new value of var {}", var);
    println!(
        "This is the value of the constant MAX_POINTS {}",
        MAX_POINTS
    );

    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("the value of x is {}", x);

    let spaces = "  ";
    let spaces = spaces.len();

    println!("the lenght of spaces is {}", spaces);

    let f = 3.0; // f64
    let y: f32 = 3.5;
}
