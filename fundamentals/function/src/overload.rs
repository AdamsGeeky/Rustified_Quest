
fn double<T: std::ops::Add<Output = T> + Copy>(x: T) -> T {
    println!("{} is {}", x, x+x)
}

fn main() {
    println!("Double of 5 is {}", double(5)); // Works with integers
    println!("Double of 3.5 is {}", double(3.5)); // Works with floats
}
