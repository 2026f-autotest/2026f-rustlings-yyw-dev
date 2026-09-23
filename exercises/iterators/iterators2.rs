// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"


//char迭代器总结
//1.&str.chars(),返回char迭代器
//2.c.next(),返回Option<char>
//3.char.to_uppercase(),把char变为大写，同时返回一个char迭代器
//4.char迭代器.chain(char迭代器)，可以把两个迭代器首位相接
//5.collect消费者方案，把迭代器所有内容塞进一个集合
//6.char迭代器.as_str()，将剩下来没有消费的部分变成&str
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(c).collect()
    }
}
// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut result: Vec<String> = vec![];
    for &str in words.iter() {
        result.push(capitalize_first(str));
    }
    result
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut result = String::new();
    let words = capitalize_words_vector(words);
    for str in words.iter() {
        result.push_str(str);
    }
    result
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
