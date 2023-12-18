use std::collections::HashSet;

pub fn source_exists(source: &str) -> bool {
    std::path::Path::new(source).exists()
}

pub fn is_unnecessary(prev: char, c: char) -> bool {
    let unnecessary_combinations: HashSet<(char, char)> = [
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

    unnecessary_combinations.contains(&(prev, c))
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
