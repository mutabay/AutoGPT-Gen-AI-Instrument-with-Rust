//! # Rust Programming Concepts
//! This module demonstrates various fundamental Rust programming concepts including:
//! - Basic types and variables
//! - Control flow
//! - Collections and data structures
//! - Bit manipulation
//! - String handling
//! - Advanced features like closures and pattern matching
mod m1_enums;
mod m2_structs;

const OUR_COURSE: &str = "Rust with AutoGPT";

fn main() {
    println!("Welcome to this course on {}", OUR_COURSE);

    // -----------------------------
    // Basic Variables and Types
    // -----------------------------
    let x: i32;
    x = 2;
    dbg!(x);

    let y: i32 = 5;
    dbg!(y);

    // -----------------------------
    // Control Flow Examples
    // -----------------------------
    for i in 0..=y {
        if i != 4 {
            dbg!(i);
        } else {
            dbg!(i);
        }
    }

    // -----------------------------
    // Mutable Variables
    // -----------------------------
    let mut z: i32 = 5;
    let mut z: i32 = 5;
    dbg!(z);
    z = 10;
    dbg!(z);

    // -----------------------------
    // Numeric Types and Operations
    // -----------------------------
    let freezing_temp: f64 = -2.4;
    dbg!(freezing_temp);

    let is_zero_remainder: bool = 10 % 4 != 0;
    dbg!(is_zero_remainder);

    // -----------------------------
    // Character Types
    // -----------------------------
    // Demonstrating regular chars and Unicode support
    let my_char: char = 'z';
    dbg!(my_char);

    let my_emoji: char = '😺';
    dbg!(my_emoji);

    // -----------------------------
    // Arrays and Collections
    // -----------------------------
    // Fixed-size array with initialization and mapping
    let my_floats: [f32; 10] = [0.0; 10];
    dbg!(my_floats);

    let my_floats_new: [f32; 10] = my_floats.map(|n| n + 2.0);
    dbg!(my_floats_new);

    // -----------------------------
    // String Types and Operations
    // -----------------------------
    let name: &str = "AutoGPT";
    dbg!(name);

    let dynamic_str: String = String::from("Hello, Rust!");
    dbg!(&dynamic_str);

    // let dynamic_str:String = name.to_string();
    // dbg!(dynamic_str);

    // let dynamic_str:String = "Anan".to_string();
    // dbg!(dynamic_str);

    let str_slice: &str = &dynamic_str[0..3];
    dbg!(&str_slice);

    // -----------------------------
    // Vector Operations
    // -----------------------------
    let mut chars: Vec<char> = Vec::new();
    chars.insert(0, 'B');
    chars.insert(0, 'C');
    chars.insert(0, 'D');
    chars.insert(0, 'E');
    dbg!(&chars);
    chars.push('A');
    dbg!(&chars);
    let popped_char: char = chars.pop().unwrap();
    dbg!(popped_char);
    let removed_char: char = chars.remove(0);
    dbg!(removed_char);

    chars.iter().for_each(|c| print!("{}", c));
    let chars_again: Vec<char> = vec!['A', 'B', 'C', 'D', 'E'];
    dbg!(&chars_again);

    for c in chars_again {
        print!("{}", c);
        if c == 'C' {
            println!("\nFound C!");
        }
    }

    // -----------------------------
    // Closures
    // -----------------------------
    // Demonstrating closure with environment capture
    let num: i32 = 5;
    let add_num = |x: i32| x + num;
    let result = add_num(10);
    dbg!(result);

    // -----------------------------
    // Number Literals and Formatting
    // -----------------------------
    println!("Big Number is: {}", 98_222_000);
    println!("Hex is {}", 0xFF);
    println!("Binary is {}", 0b1111_0000);
    println!("Octal is {}", 0o77);
    println!("Byte is {}", b'A');

    // -----------------------------
    // Advanced String Features
    // -----------------------------
    let raw_str: &str = r#"This is a raw string literal with "quotes" and \backslashes\"#;
    dbg!(raw_str);

    // String Interpolation
    let name = "AutoGPT";
    let greeting = format!("Hello, {}!", name);
    dbg!(greeting);

    // Pattern Matching
    let number = 5;
    match number {
        1 => println!("One!"),
        2 => println!("Two!"),
        3 => println!("Three!"),
        4 => println!("Four!"),
        5 => println!("Five!"),
        _ => println!("Something else!"),
    }

    // -----------------------------
    // Structs and Custom Types
    // -----------------------------
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }
    let person = Person {
        name: String::from("Alice"),
        age: 30,
    };
    dbg!(person);

    // -----------------------------
    // Binary Operations
    // -----------------------------
    let a_bin: u8 = 0b1010_1010;
    let b_bin: u8 = 0b1000_1010;
    dbg!(a_bin);
    dbg!(b_bin);
    println!("Binary a: {:08b}", a_bin);
    println!("Binary b: {:08b}", b_bin);

    // Logic gates
    let and_result = a_bin & b_bin;
    let or_result = a_bin | b_bin;
    let xor_result = a_bin ^ b_bin;
    let not_result = !a_bin;
    print!("AND: {:08b}\n", and_result);
    print!("OR: {:08b}\n", or_result);
    print!("XOR: {:08b}\n", xor_result);
    print!("NOT: {:08b}\n", not_result);

    // -----------------------------
    // Bit Manipulation
    // -----------------------------
    let left_shift = a_bin << 2;
    let right_shift = a_bin >> 2;
    println!("Left Shift: {:08b}", left_shift);
    println!("Right Shift: {:08b}", right_shift);
    // Bit masks
    let mask: u8 = 0b1111_0000;
    let masked_value = a_bin & mask;
    println!("Masked Value: {:08b}", masked_value);

    // -----------------------------
    // Endianness
    // -----------------------------
    let little_endian: u16 = 0x1234;
    let big_endian: u16 = 0x3412;
    println!("Little Endian: {:04x}", little_endian);
    println!("Big Endian: {:04x}", big_endian);
    // Endianness check
    if cfg!(target_endian = "little") {
        println!("This system is Little Endian");
    } else {
        println!("This system is Big Endian");
    }

    // Enums
    #[derive(Debug)]
    enum Status {
        Active,
        Inactive,
        Pending,
    }
    
    impl ToString for Status {
    fn to_string(&self) -> String {
        match self {
            Status::Active => String::from("Active"),
            Status::Inactive => String::from("Inactive"),
            Status::Pending => String::from("Pending"),
        }
    }
    }

    let status = Status::Active;
    let status_string = status.to_string();
    println!("Status as string: {}", status_string);

    // Using the m1_enums module
    let car_color = m1_enums::handle_car_color();
    dbg!(car_color);
    

}
