fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // Safe access using get()
    match numbers.get(10) {
        Some(num) => println!("The number is: {}", num),
        None => println!("Index out of bounds"),
    }

    // Check bounds before accessing
    if numbers.len() > 10 {
        let num = numbers[10];
        println!("The number is: {}", num);
    } else {
        println!("Index out of bounds");
    }
} 