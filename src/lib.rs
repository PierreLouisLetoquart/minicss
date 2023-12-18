use std::collections::HashSet;

pub fn source_exists(source: &str) -> bool {
    std::path::Path::new(source).exists()
}

pub fn is_unnecessary(prev: char, c: char) -> bool {
    let it_is: HashSet<(char, char)> = [
        (';', ' '),
        ('{', ' '),
        ('}', ' '),
        ('(', ' '),
        (')', ' '),
        (',', ' '),
        (':', ' '),
        ('=', ' '),
        ('+', ' '),
        ('-', ' '),
        ('*', ' '),
        ('/', ' '),
        ('%', ' '),
        ('!', ' '),
        ('>', ' '),
        ('<', ' '),
        ('&', ' '),
        ('|', ' '),
        ('^', ' '),
        ('~', ' '),
        ('[', ' '),
        (']', ' '),
        (' ', ' '),
    ]
    .iter()
    .cloned()
    .collect();

    it_is.contains(&(prev, c))
}

pub fn minify_content(content: &str) -> String {
    let mut minified = Vec::new();
    let mut chars = content.chars();
    let mut prev = ' ';

    while let Some(c) = chars.next() {
        if c == '\n' {
            continue;
        }

        if c == ' ' && is_unnecessary(prev, c) {
            continue;
        }

        if c == '/' {
            if let Some('*') = chars.next() {
                while let Some(c) = chars.next() {
                    if c == '*' {
                        if let Some('/') = chars.next() {
                            break;
                        }
                    }
                }
                continue;
            }
        }

        minified.push(c);
        prev = c;
    }

    minified.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_exists() {
        assert_eq!(source_exists("src/lib.rs"), true);
        assert_eq!(source_exists("src/lib2.rs"), false);
    }

    #[test]
    fn test_is_unnecessary() {
        assert_eq!(is_unnecessary(' ', ' '), true);
        assert_eq!(is_unnecessary(' ', ';'), false);
        assert_eq!(is_unnecessary(' ', '{'), false);
        assert_eq!(is_unnecessary('%', ' '), true);
    }

    #[test]
    fn test_minify_content() {
        assert_eq!(
            minify_content(
                r#"
                /*
                 * This is a multi-line comment.
                 */
                
                * {
                  margin: 0;
                }

                html,
                body {
                  /* height: 100%; */
                  height: 100%;
                }
            "#
            ),
            r#"*{margin:0;}html,body {height:100%;}"#
        );
    }
}
