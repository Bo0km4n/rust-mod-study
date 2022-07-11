mod dog;
mod cat;
fn main() {
    println!("Hello, world!");
    dog::dog::call();
    cat::cat::call();
}
