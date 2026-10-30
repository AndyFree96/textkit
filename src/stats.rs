#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileStats {
    pub lines: usize,
    pub words: usize,
    pub characters: usize,
    pub bytes: usize,
}

pub fn count_text(content: &str) -> FileStats {
    FileStats {
        lines: content.lines().count(),
        words: content.split_whitespace().count(),
        characters: content.chars().count(),
        bytes: content.len(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counts_text() {
        let stats = count_text("Rust is fast.\n你好");

        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 4);
        assert_eq!(stats.characters, 16);
        assert_eq!(stats.bytes, 20);
    }

    #[test]
    fn handles_empty_text() {
        assert_eq!(
            count_text(""),
            FileStats {
                lines: 0,
                words: 0,
                characters: 0,
                bytes: 0,
            }
        );
    }

    #[test]
    fn counts_unicode_characters_and_utf8_bytes() {
        let stats = count_text("你好\nRust");

        assert_eq!(stats.lines, 2);
        assert_eq!(stats.words, 2);
        assert_eq!(stats.characters, 7);
        assert_eq!(stats.bytes, 11);
    }

    #[test]
    fn trailing_newline_does_not_add_an_extra_line() {
        let stats = count_text("one\ntwo\n");

        assert_eq!(stats.lines, 2);
    }
}
