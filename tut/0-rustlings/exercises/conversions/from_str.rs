// This does practically the same thing that TryFrom<&str> does.
// Additionally, upon implementing FromStr, you can use the `parse` method
// on strings to generate an object of the implementor type.
// You can read more about it at https://doc.rust-lang.org/std/str/trait.FromStr.html
use std::str::FromStr;

#[derive(Debug)]
struct Person {
    name: String,
    age: usize,
}

// I AM NOT DONE
// Steps:
// 1. Split the given string on the commas present in it
// 2. Extract the first element from the split operation and use it as the name
// 3. Extract the other element from the split operation and parse it into a `usize` as the age
// If something goes wrong, for instance there is no comma in the provided string or
// parsing the age fails, then return Err of String
// Otherwise, return Ok result of a Person object
impl FromStr for Person {
    type Err = String;
    fn from_str(s: &str) -> Result<Person, Self::Err> {
        if s.len() == 0 {
            return Err(String::from("Empty String."));
        }
        
        let v = s.split(",").collect::<Vec<&str>>();
        
        if v.len() != 2 {
            return Err(String::from("String is incomplete."));
        }

        match v[1].parse::<usize>() {
            Ok(age) => Ok(
                Person {
                    name: String::from(v[0]),
                    age: age,
                }
            ),
            Err(_) => Err(String::from("Failed to parse age."))
        }
    }
}

fn main() {
    let p = "Mark,20".parse::<Person>().unwrap();
    println!("{:?}", p);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input() {
        assert!("".parse::<Person>().is_err());
    }
    #[test]
    fn good_input() {
        assert!("John,32".parse::<Person>().is_ok());
    }
    #[test]
    #[should_panic]
    fn missing_age() {
        "John".parse::<Person>().unwrap();
    }
}
