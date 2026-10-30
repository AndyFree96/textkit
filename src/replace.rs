/// 替换文本中所有不重叠的匹配项
/// 
/// 返回替换后的字符串和实际替换次数
/// 空关键字不执行替换
pub fn replace_text(content: &str, pattern: &str, replacement: &str) -> (String, usize) {
    if pattern.is_empty() {
        return (content.to_owned(), 0);
    }

    let count = content.match_indices(pattern).count();
    let result = content.replace(pattern, replacement);
    (result, count)
}

#[cfg(test)]
mod replace_tests {
    use super::*;

    #[test]
    fn replaces_all_matches() {
        let (result, count) = replace_text("Rust is fast. Rust is fun", "Rust", "Go");
        assert_eq!(result, "Go is fast. Go is fun");
        assert_eq!(count, 2);
    }

    #[test]
    fn returns_original_when_no_match() {
        let (result, count) = replace_text("Hello World", "Rust", "Go");
        assert_eq!(result, "Hello World");
        assert_eq!(count, 0);
    }

    #[test]
    fn handle_empty_pattern() {
        let (result, count) = replace_text("Hello", "", "Rust");
        assert_eq!(result, "Hello");
        assert_eq!(count, 0);
    }

    #[test]
    fn supports_unicode() {
        let (result, count) = replace_text("你好，Rust! Rust很棒！", "Rust", "Go");
        assert_eq!(result, "你好，Go! Go很棒！");
        assert_eq!(count, 2);
    }
}
