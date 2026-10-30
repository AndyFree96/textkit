/// 过滤模式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterMode {
    Contains,
    Exclude,
    StartsWith,
}

/// 根据过滤模式筛选文本行
/// 返回的含内容借用自原始文本，不复制每一行
pub fn filter_lines<'a>(
    content: &'a str,
    keyword: &str,
    mode: FilterMode,
) -> Vec<(usize, &'a str)> {
    if keyword.is_empty() {
        return Vec::new();
    }

    content
        .lines()
        .enumerate()
        .filter(|(_, line)| match mode {
            FilterMode::Contains => line.contains(keyword),
            FilterMode::Exclude => !line.contains(keyword),
            FilterMode::StartsWith => line.starts_with(keyword),
        })
        .map(|(index, line)| (index + 1, line))
        .collect()
}

#[cfg(test)]
mod filter_tests {
    use super::*;

    const CONTENT: &str = "Rust is fast.\nPython is readable.\nRust is safe.\nGo is interesting.";

    #[test]
    fn keeps_matching_lines() {
        let results = filter_lines(CONTENT, "Rust", FilterMode::Contains);

        assert_eq!(results, vec![(1, "Rust is fast."), (3, "Rust is safe.")]);
    }

    #[test]
    fn excludes_matching_lines() {
        let results = filter_lines(CONTENT, "Rust", FilterMode::Exclude);

        assert_eq!(
            results,
            vec![(2, "Python is readable."), (4, "Go is interesting.")]
        );
    }

    #[test]
    fn keeps_lines_start_with_keyword() {
        let results = filter_lines(CONTENT, "Rust", FilterMode::StartsWith);

        assert_eq!(results, vec![(1, "Rust is fast."), (3, "Rust is safe.")]);
    }

    #[test]
    fn empty_keyword_returns_no_results() {
        let results = filter_lines(CONTENT, "", FilterMode::Contains);

        assert!(results.is_empty());
    }
}
