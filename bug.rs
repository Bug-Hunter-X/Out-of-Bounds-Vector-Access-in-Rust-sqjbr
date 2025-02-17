fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let second_element = vec.get(1).unwrap();
    println!("The second element is: {}", second_element);
    //Attempt to access index 2, which is out of bounds
    let third_element = vec.get(2).unwrap();
    println!("The third element is: {}", third_element);
}