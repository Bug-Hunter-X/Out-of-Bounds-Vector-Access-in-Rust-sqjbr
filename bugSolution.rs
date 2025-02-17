fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    // Safe way to access elements using get()
    if let Some(second_element) = vec.get(1) {
        println!("The second element is: {}", second_element);
    } else {
        println!("Index out of bounds");
    }

    if let Some(third_element) = vec.get(2) {
        println!("The third element is: {}", third_element);
    } else {
        println!("Index out of bounds");
    }
}