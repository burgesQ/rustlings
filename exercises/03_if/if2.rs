const BAD: [&str; 3] = ["broccoli", "gummy bears", "literally anything"];

// TODO: Fix the compiler error on this function.
fn picky_eater(food: &str) -> &str {

  //  let bad = ["broccoli", "gummy bears", "literally anything"];

    if food == "strawberry" {
        "Yummy!"
    } else if food == "potato" {
        "I guess I can eat that."
    } else if BAD.contains(&food) {
        "No thanks!"
    } else {
        food
    }
}

fn main() {
    // You can optionally experiment here.
}

// TODO: Read the tests to understand the desired behavior.
// Make all tests pass without changing them.
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn yummy_food() {
        // This means that calling `picky_eater` with the argument "food" should return "Yummy!".
        assert_eq!(picky_eater("strawberry"), "Yummy!");
    }

    #[test]
    fn neutral_food() {
        assert_eq!(picky_eater("potato"), "I guess I can eat that.");
    }

    #[test]
    fn default_disliked_food() {
        assert_eq!(picky_eater("broccoli"), "No thanks!");
        assert_eq!(picky_eater("gummy bears"), "No thanks!");
        assert_eq!(picky_eater("literally anything"), "No thanks!");
    }
}
