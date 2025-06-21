#[allow(dead_code, unused_variables)]

fn example_0() {
    let r: &i32;

    let x: i32 = 5;
    r = &x;

    println!("r: {}", r);
}

struct Person<'p, 'q> {
    name: &'p str,
    points: &'q f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lifetimes() {
        // Allocate space in memory
        let highest_age: &i32;
        let new_value: i32;
        // Initialize vars
        let alice_age: i32 = 20;

        {
            let bob_age: i32 = 21;
            // Call function
            highest_age = largest(&alice_age, &bob_age);
            new_value = *highest_age;
        } // 'b out of scope

        dbg!(new_value);

        fn largest<'a>(compare_1: &'a i32, compare_2: &'a i32) -> &'a i32 {
            if compare_1 > compare_2 {
                compare_1
            } else {
                compare_2
            }
        }
    }

    #[test]
    fn test_lifetimes_generic() {
        // Allocate space in memory
        let highest_age: &i32;
        let new_value: i32;
        // Initialize vars
        let alice_age: i32 = 20;

        {
            let bob_age: i32 = 21;
            // Call function
            highest_age = largest::<i32>(&alice_age, &bob_age);
            new_value = *highest_age;
        } // 'b out of scope

        dbg!(new_value);

        fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
            if compare_1 > compare_2 {
                compare_1
            } else {
                compare_2
            }
        }
    }

    #[test]
    fn test_lifetimes_struct() {
        // Allocate space in memory
        let highest_age: &f32;
        let new_value: f32;
        // Initialize vars
        let alice: Person = Person {
            name: "Alice",
            points: &50.2,
        };

        {
            let bob: Person = Person {
                name: "Bob",
                points: &40.5,
            };
            // Call function
            highest_age = largest::<f32>(alice.points, &*bob.points);
            new_value = *highest_age;
        } // 'b out of scope

        dbg!(new_value);

        fn largest<'a, T: PartialOrd>(compare_1: &'a T, compare_2: &'a T) -> &'a T {
            if compare_1 > compare_2 {
                compare_1
            } else {
                compare_2
            }
        }
    }
}
