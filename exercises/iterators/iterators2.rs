// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// Step 1.
// 完成 `capitalize_first` 函数。
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(c).collect(), // 将第一个字符大写，然后连接剩余字符并收集为字符串
    }
}

// Step 2.
// 将 `capitalize_first` 函数应用到字符串切片的切片。
// 返回一个字符串向量。
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|&word| capitalize_first(word)).collect() // 对每个单词调用 capitalize_first 函数，然后收集为字符串向量
}

// Step 3.
// 再次将 `capitalize_first` 函数应用到字符串切片的切片。
// 返回一个单一字符串。
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|&word| capitalize_first(word)).collect::<Vec<String>>().join("") // 对每个单词调用 capitalize_first 函数，然后收集为字符串向量，最后连接成单一字符串
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
