fn main() {
    //Compound Data Types
    //arrays, tuples, slices, string

    //Arrays
    //Array is a fixed size collection of elements of the same type
    let number: [i32; 5] = [1, 2, 3, 4, 5];
    println!("Number Array: {:?}", number);
    let fruits: [&str; 4] = ["apple", "banana", "orange","🍑"];
    println!("Fruits Array: {:?}", fruits);
    println!("Second fruit is a : {}", fruits[1]);

    //Tuples
    let human: (String, i32, bool) = ("John".to_string(), 30, true);
    let mix_human  = (human.clone(),[1,2,3,4,5], 22, true);
    println!("Human: {:?}", human);
    println!("Mix Human: {:?}", mix_human);
    // //Slices
    let slice: &[i32] = &[1, 2, 3, 4, 5];
    let slice2: &[&str] = &["Lions", "Tigers", "Bears"];
    let slice3: &[&String] = &[&"John".to_string(), &"Jack".to_string(), &"Jill".to_string()];

    println!("Slice: {:?}", slice);
    println!("Slice2: {:?}", slice2);
    println!("Slice3: {:?}", slice3);

    //Strings  vs StringSlice (&str)
    let mut string = String::from("Hello World");
    println!("String: {:?}", string);
    string.push_str("!");
    println!("String: {:?}", string);

    //B- &str (string slice)
    let string_slice: &str = "Hello, World";
    let string_slice2: &str = &string[0..5];
    println!("String Slice: {:?}", string_slice);
    println!("String Slice: {:?}", string_slice2);
}