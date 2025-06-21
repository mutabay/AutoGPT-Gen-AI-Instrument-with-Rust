#[derive(Debug)]
enum Message {
    Quit,
    ChangeColor(i32, i32, i32),
    Move { x: i32, y: i32 },
    Write(String),
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("I quit!");
        }
        Message::ChangeColor(red, green, blue) => {
            println!("Red {}, Green {}, Blue {}", red, green, blue);
        }
        Message::Move { x, y: new_name } => {
            println!("X is {}, Y as new_name is {}", x, new_name);
        }
        Message::Write(text) => {
            println!("{}", text);
        }
    };
}

#[cfg(test)]
mod tests {
    use core::error;

    use ethers::types::Res;

    use super::*;

    #[test]
    fn test_match_literals() {
        let number: i32 = 20;

        let res: &str = match number {
            1 => "This is the first!",
            2 | 3 | 5 | 7 | 15 | 20 => "We found it in the sequence!",
            _ => "It was something else!",
        };

        println!("{}", res);
    }

    #[test]
    fn test_match_option() {
        let some_num: Option<i32> = Some(10);
        let prob_none: Option<i32> = None;

        let my_int: i32 = if let Some(i) = some_num {
            i
        } else {
            panic!("There was a problem");
        };

        println!("my_int value is: {}", my_int);

        let res = match some_num {
            Some(i) => i,
            None => panic!("There was a problem"),
        };
        println!("{:?}", some_num);
        println!("{}", res);
    }

    #[test]
    fn test_match_result() {
        let some_res: Result<i32, &str> = Ok(50);
        let some_err: Result<i32, &str> = Err("Thre was a problem");

        let res = match some_err {
            Ok(val) => val,
            Err(e) => panic!("{}", e),
        };

        println!("{}", res);

        let my_int: i32 = if let Ok(i) = some_res {
            i
        } else {
            panic!("There was a problem");
        };

        println!("my_int value is: {}", my_int);
    }

    #[test]
    fn test_large_enum() {
        let my_quit = Message::Quit;
        let my_color: Message = Message::ChangeColor(10, 20, 30);
        let my_move: Message = Message::Move { x: 10, y: 200 };
        let my_write: Message = Message::Write("Wasssuuup!!".to_string());
        process_message(my_write);
    }
}
