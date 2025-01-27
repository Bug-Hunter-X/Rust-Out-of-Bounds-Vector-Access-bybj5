fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let num = numbers[10]; // Out of bounds
    println!("The number is: {}", num);
}