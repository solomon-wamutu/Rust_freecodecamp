
// fn main() {
//     let my_string = String::from("The quick brown fox");
// let my_str: &str = &my_string[4..9]; // "quick"

// let my_arr: [usize; 5] = [1, 2, 3, 4, 5];
// let my_arr_slice: &[usize] = &my_arr[0..3]; // [1, 2, 3]
//     println!("{} {} {:?} {:?}",my_string,my_str,my_arr,my_arr_slice);
// }




// struct MyString {
//     vec: Vec<u8>,
// }

// impl MyString {
//     // Custom implementation of `from` method
//     fn from(s: &str) -> Self {
//         MyString {
//             vec: Vec::from(s.as_bytes()),
//         }
//     }
    
//     // Method to convert MyString back to &str
//     fn as_str(&self) -> &str {
//         std::str::from_utf8(&self.vec).unwrap()
//     }
// }

// fn main() {
//     let my_string = MyString::from("Hello, world!");
//     println!("{}", my_string.as_str()); // Output: Hello, world!
// }



// fn main() { // first_string is not declared yet -> has no value
//     let first_string = String::from("freeCodeCamp"); // first_string is now owner of the value "freeCodeCamp"
//     let second_string = &first_string; // second_string takes ownership of the value "freeCodeCamp"
  
//     println!("Hello, {} {}!", first_string,second_string); // first_string is NOT valid, because the value was moved to second_string
//   }

// fn main() {
//   my_func();

// }
// fn my_func() -> u8 {
// 0}

// fn main() {
//   let used_variable = my_func(10);
//   println!("{}, ",used_variable)
// }

// fn my_func(x: u8) -> i32 {
//   x as i32
// }

fn main() {
  let my_str: &str = "Hello, world!";

  let my_string: String = String::from("Hello, world!");
  println!("{} , {}", my_str,my_string)
  
}