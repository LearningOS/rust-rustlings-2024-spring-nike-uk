// from_str.rs
//
// This is similar to from_into.rs, but this time we'll implement `FromStr` and
// return errors instead of falling back to a default value. Additionally, upon
// implementing FromStr, you can use the `parse` method on strings to generate
// an object of the implementor type. You can read more about it at
// https://doc.rust-lang.org/std/str/trait.FromStr.html
//
// Execute `rustlings hint from_str` or use the `hint` watch subcommand for a
// hint.

use std::convert::TryFrom;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct Color {
    red: i16,
    green: i16,
    blue: i16,
}

#[derive(Debug)]
struct ParseColorError;

impl TryFrom<(i16, i16, i16)> for Color {
    type Error = ParseColorError;

    fn try_from(tuple: (i16, i16, i16)) -> Result<Self, Self::Error> {
        let (red, green, blue) = tuple;
        if red < 0 || green < 0 || blue < 0 {
            return Err(ParseColorError);
        }
        Ok(Color { red, green, blue })
    }
} 

impl TryFrom<[i16; 3]> for Color {
    type Error = ParseColorError;

    fn try_from(arr: [i16; 3]) -> Result<Self, Self::Error> {
        let [red, green, blue] = arr;
        if red < 0 || green < 0 || blue < 0 {
            return Err(ParseColorError);
        }
        Ok(Color { red, green, blue })
    }
}

impl TryFrom<&[i16]> for Color {
    type Error = ParseColorError;

    fn try_from(slice: &[i16]) -> Result<Self, Self::Error> {
        if slice.len() != 3 {
            return Err(ParseColorError);
        }
        let red = slice[0];
        let green = slice[1];
        let blue = slice[2];
        if red < 0 || green < 0 || blue < 0 {
            return Err(ParseColorError);
        }
        Ok(Color { red, green, blue })
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<i16> = s
            .split(',')
            .filter_map(|c| c.parse::<i16>().ok())
            .collect();

        if components.len() != 3 {
            return Err(ParseColorError);
        }

        Color::try_from(components.as_slice())
    }
}

fn main() {
    let tuple_color = (100, 150, 200);
    let color: Color = tuple_color.try_into().unwrap();
    println!("{:?}", color);

    let arr_color: Color = [50, 100, 150].try_into().unwrap();
    println!("{:?}", arr_color);

    let slice_color: Color = [25, 50, 75][..].try_into().unwrap();
    println!("{:?}", slice_color);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_tuple() {
        let color = Color::try_from((100, 150, 200)).unwrap();
        assert_eq!(color, Color { red: 100, green: 150, blue: 200 });
    }

    #[test]
    fn test_try_from_array() {
        let color = Color::try_from([50, 100, 150]).unwrap();
        assert_eq!(color, Color { red: 50, green: 100, blue: 150 });
    }

    #[test]
    fn test_try_from_slice() {
        let color = Color::try_from(&[25, 50, 75][..]).unwrap();
        assert_eq!(color, Color { red: 25, green: 50, blue: 75 });
    }

    #[test]
    fn test_try_from_tuple_negative() {
        let result = Color::try_from((-1, 150, 200));
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_array_negative() {
        let result = Color::try_from([-1, 100, 150]);
        assert!(result.is_err());
    }

    #[test]
    fn test_try_from_slice_negative() {
        let result = Color::try_from(&[-1, 50, 75][..]);
        assert!(result.is_err());
    }

    #[test]
    fn test_from_str() {
        let color = "100,150,200".parse::<Color>().unwrap();
        assert_eq!(color, Color { red: 100, green: 150, blue: 200 });
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("100,150".parse::<Color>().is_err());
        assert!("100,150,200,250".parse::<Color>().is_err());
    }
}
