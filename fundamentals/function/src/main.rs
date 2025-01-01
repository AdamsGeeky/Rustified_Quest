mod user_input;

fn add(a: i32, b: i32)-> i32{
    a + b
}

fn total(element: &[i32]) -> i32{
    element.iter().sum()
}

fn square(n: i64)->i64{
    n * n
}

//  defualt return unit type ()
fn display_message(message: &str) {
    println!("{}", message);
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}

fn main() {
    let name = user_input::user_name();
    greet(&name);
    display_message("Rust is awesome !");
    let numbers = [1, 2, 3, 4, 5,6,7,8,9];
    println!("Welcome to Sum: {}", add(7,3));
    println!("the Total Sum is = {}", total(&numbers));
    println!("the Square of 12345 is {}", square(12345))

}
