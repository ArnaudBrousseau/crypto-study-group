use generators::find_generator;

pub fn main() {
    // Answers to the homework
    println!("Generator for Z/1009Z: {}", find_generator(1009));
    println!("Generator for Z/2357Z: {}", find_generator(2357));
}
