#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use super::*;

    #[test]
    fn test_hashmap() {
        
        let person_1: &str= "Alice";
        let person_2: &str= "Bob";
        
        // &str --> Person
        // u8 --> &str
        // &str --> u32

        let mut results_hm: HashMap<&str, u32> = HashMap::new();
        results_hm.insert(person_1, 55);
        results_hm.insert(person_2, 51);

        let test_res = results_hm.get(person_1);
        dbg!(test_res);
    }

    #[test]
    fn test_hashset() {
        let mut names_hs: HashSet<&str> = HashSet::new();
        names_hs.insert("Alice");
        names_hs.insert("Bob"); 

        let test_res = names_hs.contains("Alice");
        
        dbg!(test_res);

    }
}