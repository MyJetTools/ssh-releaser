pub fn convert_url_encoded(content: &str) -> String {
    let mut result = String::new();

    for c in content.chars() {
        match c {
            ':' => result.push_str("%3A"),
            '\n' => result.push_str("%0A"),
            ' ' => result.push_str("+"),
            '!' => result.push_str("%21"),
            '"' => result.push_str("%22"),
            '#' => result.push_str("%23"),
            '$' => result.push_str("%24"),
            '%' => result.push_str("%25"),
            '&' => result.push_str("%26"),
            '\'' => result.push_str("%27"),
            '(' => result.push_str("%28"),
            ')' => result.push_str("%29"),
            '*' => result.push_str("%2A"),
            '+' => result.push_str("%2B"),
            ',' => result.push_str("%2C"),
            '-' => result.push_str("%2D"),
            '.' => result.push_str("%2E"),
            '/' => result.push_str("%2F"),
            ';' => result.push_str("%3B"),
            '<' => result.push_str("%3C"),
            '=' => result.push_str("%3D"),
            '>' => result.push_str("%3E"),
            '?' => result.push_str("%3F"),
            '@' => result.push_str("%40"),
            '[' => result.push_str("%5B"),
            '\\' => result.push_str("%5C"),
            ']' => result.push_str("%5D"),
            '^' => result.push_str("%5E"),
            '_' => result.push_str("%5F"),
            '`' => result.push_str("%60"),
            '{' => result.push_str("%7B"),
            '|' => result.push_str("%7C"),
            '}' => result.push_str("%7D"),
            '~' => result.push_str("%7E"),
            _ => result.push(c),
        }
    }

    result
}
