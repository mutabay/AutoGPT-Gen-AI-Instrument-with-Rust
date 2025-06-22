#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};

    use super::*;

    #[test]
    fn test_box_smart_pointer() {
        // Box<T> is a smart pointer that allocates memory on the heap
        // It is used to store data on the heap and provides ownership and automatic cleanup
        // Good for recursive data structures or large data that you want to allocate on the heap
        #[derive(Debug)]
        struct Node {
            id: u32,
            next: Option<Box<Node>>,
        }

        let nodes: Box<Node> = Box::new(Node {
            id: 0,
            next: Some(Box::new(Node {
                id: 1,
                next: Some(Box::new(Node { id: 2, next: None })),
            })),
        });

        dbg!(nodes);
    }

    #[test]
    fn test_reference_counter() {
        // let mut x = Rc::new(50); // Rc<T> is a reference-counted smart pointer
        let mut x = Rc::new(RefCell::new(50)); // Rc<T> is a reference-counted smart pointer
        let y = Rc::clone(&x); // y is a reference to x (not deep copy, just a reference counter increment)

        *x.borrow_mut() = 70; // x is mutable, so we can change its value

        dbg!(x);
        dbg!(y);

        #[derive(Debug, Clone)]
        struct House {
            address_number: u16,
            street: String,
            furniture: RefCell<Vec<Rc<Furniture>>>,
        }

        #[derive(Debug, Clone)]
        struct Furniture {
            id: String,
            description: String,
            house: Weak<House>,
        }

        let house_1 = Rc::new(House {
            address_number: 123,
            street: "coding avenue".to_string(),
            furniture: RefCell::new(vec![]),
        });

        let table = Rc::new(Furniture {
            id: "table1".to_string(),
            description: "kitchen table".to_string(),
            house: Rc::downgrade(&house_1),
        });

        let desk = Rc::new(Furniture {
            id: "desk1".to_string(),
            description: "office desk".to_string(),
            house: Rc::downgrade(&house_1),
        });

        house_1.furniture.borrow_mut().push(Rc::clone(&table));
        house_1.furniture.borrow_mut().push(Rc::clone(&desk));
        dbg!(house_1);
    }
}
