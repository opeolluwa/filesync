fn main() {
    // Declare string
    let string: String = String::from("Some string");

    // declare a string slice
    let string_slice: &str = "some string slice";

    // convert string to string slice
    let new_string_slice = string.as_str();

    // convert string slice to string
    let new_string = string_slice.to_string();

    println!("\"{new_string}\" is now a string and \"{new_string_slice}\" is a string slice");
     //"some string slice" is now a string and "Some string" is a string slice
}
