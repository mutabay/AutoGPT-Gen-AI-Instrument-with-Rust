use core::num;

/*

Use Result when an operation can fail with an error
Use Option when a value might not exist
Result has Ok and Err variants
Option has Some and None variants

*/
#[derive(Debug)]
pub enum CarColor {
    Red,
    Green,
    Blue,
    Yellow,
    Black,
}

pub fn handle_car_color() -> CarColor {
    let my_car_color: CarColor = CarColor::Blue;
    println!("The car color is: {:?}", my_car_color);
    my_car_color
}

#[derive(Debug)]
enum GivenResult<T, E> {
    Ok(T),
    Err(E),
}

fn check_under_five(num: u8) -> GivenResult<u8, String> {
    if num < 5 {
        GivenResult::Ok(num)
    } else {
        GivenResult::Err("Number is 5 or greater".to_string())
    }
}

fn check_under_five_built_in(num: u8) -> Result<u8, String> {
    if num < 5 {
        Ok(num)
    } else {
        Err("Number is 5 or greater".to_string())
    }
}

#[derive(Debug)]
enum GivenOption<T> {
    Some(T),
    None
}

fn remainder_zero(num_check: f32) -> GivenOption<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        GivenOption::Some(num_check)
    } else {
        GivenOption::None
    }

}
fn remainder_zero_built_in(num_check: f32) -> Option<f32> {
    let remainder: f32 = num_check % 10.0;
    if remainder != 0.0 {
        Some(remainder)
    } else {
        None
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_enums() {
        let car_color = handle_car_color();
        dbg!(car_color);

        let under_five = check_under_five(3);
        dbg!(under_five);
        let under_five = check_under_five(7);
        dbg!(under_five);
        
        let under_five = check_under_five_built_in(1);
        dbg!(under_five);
        
        let remainder_check = remainder_zero(12.2);
        dbg!(remainder_check);

        let remainder_check = remainder_zero_built_in(12.2);
        dbg!(remainder_check);

    }
}
