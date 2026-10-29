/// 搜索包含指定关键字的文本行
/// 
/// 返回 (行号， 行内容)，行号从1开始
/// 行内容借用自原始文本，不会复制每一行的字符串

pub fn search_lines<'a>(content: &'a str, keyword: &str) -> Vec<(usize, &'a str)> {
    if keyword.is_empty() {
        return Vec::new();
    }

    content.lines().enumerate().filter(|(_, line)| line.contains(keyword))
            .map(|(index, line)| (index + 1, line)).collect()
}

#[cfg(test)]
mod tests {
    use super::search_lines;

    #[test]
    fn finds_matching_lines() {
        let content = "Rust is fast.\nPython is readable.\nI like Rust.";
        let results = search_lines(content, "Rust");

        assert_eq!(results, vec![(1, "Rust is fast."), (3, "I like Rust.")]);
    }

    #[test]
    fn returns_empty_when_no_match() {
        let results = search_lines("Hello\nWorld", "Rust");

        assert!(results.is_empty());
    }

    #[test]
    fn ignores_empty_keyword() {
        let results = search_lines("Hello\nWorld", "");

        assert!(results.is_empty());
    }
}