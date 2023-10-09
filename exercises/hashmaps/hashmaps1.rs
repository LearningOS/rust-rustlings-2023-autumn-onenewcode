

use std::collections::HashMap;

fn fruit_basket() -> HashMap<String, u32> {
    let mut basket = HashMap::new();

    // Two bananas are already given for you :)
    basket.insert(String::from("banana"), 2);
    basket.insert(String::from("banagfa"), 2);
    basket.insert(String::from("bhjkana"), 2);
    basket.insert(String::from("bandsga"), 2);
    basket.insert(String::from("bagdfsa"), 2);

    // TODO: Put more fruits in your basket here.

    basket
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn at_least_three_types_of_fruits() {
        let basket = fruit_basket();
        assert!(basket.len() >= 3);
    }

    #[test]
    fn at_least_five_fruits() {
        let basket = fruit_basket();
        assert!(basket.values().sum::<u32>() >= 5);
    }
}
