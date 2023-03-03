use std::io;

fn main() {
    println!("Hi, it is a dot product calculator!");
    println!("Enter the number of the first vector(one for each line):");
    let mut x1_input = String::new();
    io::stdin().read_line(&mut x1_input).unwrap();
    let x1: f32 = x1_input.trim().parse().unwrap();

    let mut x2_input = String::new();
    io::stdin().read_line(&mut x2_input).unwrap();
    let x2: f32 = x2_input.trim().parse().unwrap();

    println!("Enter the number of the second vector(one for each line):");
    let mut y1_input = String::new();
    io::stdin().read_line(&mut y1_input).unwrap();
    let y1: f32 = y1_input.trim().parse().unwrap();

    let mut y2_input = String::new();
    io::stdin().read_line(&mut y2_input).unwrap();
    let y2: f32 = y2_input.trim().parse().unwrap();

    let dot_product = x1 * y1 + x2 * y2;
    println!("The dot product of the vectors is: {}", dot_product);
}