pub fn escape_string_value(value: &str) -> String {
    let mut escaped_value = String::with_capacity(value.len());

    for value_char in value.chars() {
        match value_char {
            '\\' => {
                escaped_value.push('\\');
                escaped_value.push('\\');
            }
            '"' => {
                escaped_value.push('\\');
                escaped_value.push('"');
            }
            '\n' => {
                escaped_value.push('\\');
                escaped_value.push('n');
            }
            '\r' => {
                escaped_value.push('\\');
                escaped_value.push('r');
            }
            '\t' => {
                escaped_value.push('\\');
                escaped_value.push('t');
            }
            c if c.is_ascii_control() => {
                escaped_value.push('\\');
                escaped_value.push('x');
                escaped_value.push_str(format!("{:02x}", c as u8).as_str());
            }
            c => escaped_value.push(c),
        }
    }

    escaped_value
}

pub fn unescape_string_value(escaped_value: &str) -> Option<String> {
    let mut value = String::with_capacity(escaped_value.len());
    let mut escaped_value_chars = escaped_value.chars();

    while let Some(escaped_value_char) = escaped_value_chars.next() {
        let value_char = if escaped_value_char == '\\' {
            match escaped_value_chars.next()? {
                '\\' => '\\',
                '"' => '"',
                'n' => '\n',
                'r' => '\r',
                't' => '\t',
                'x' => {
                    let mut escaped_value_char_hex = String::with_capacity(2);
                    escaped_value_char_hex.push(escaped_value_chars.next()?);
                    escaped_value_char_hex.push(escaped_value_chars.next()?);
                    u8::from_str_radix(escaped_value_char_hex.as_str(), 16).ok()? as char
                }
                _ => return None,
            }
        } else {
            escaped_value_char
        };

        value.push(value_char);
    }

    Some(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::prelude::*;

    #[test]
    fn should_escape_string_value() {
        let target = "test\t\"value\"\r\n\\\x1b";
        let result = escape_string_value(target);

        assert_that!(result, eq("test\\t\\\"value\\\"\\r\\n\\\\\\x1b"));
    }

    #[test]
    fn should_unescape_string_value() {
        let target = "test\\t\\\"value\\\"\\r\\n\\\\\\x1b";
        let result = unescape_string_value(target);

        assert_that!(
            result,
            matches_pattern!(Some(eq("test\t\"value\"\r\n\\\x1b")))
        );
    }
}
