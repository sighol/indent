use std::iter::Peekable;
use std::str::Chars;

pub fn indent(input: &str) -> String {
    let input = input.to_string();

    let mut output = String::new();
    let mut indent = 0;

    let mut is_inside_string = false;

    let mut chars = input.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\'' || c == '"' {
            is_inside_string = !is_inside_string;
            output.push(c);
            continue;
        }

        if is_inside_string {
            output.push(c);
            continue;
        }

        if c == '\n' {
            eat_whitespace(&mut chars);
        } else if c == ',' {
            output.push(c);
            newline(indent, &mut chars, &mut output);
        } else if is_open(c) {
            indent += 1;
            output.push(c);
            newline(indent, &mut chars, &mut output);
        } else if is_close(c) {
            indent -= 1;
            newline(indent, &mut chars, &mut output);
            output.push(c);
        } else {
            output.push(c);
        }
    }

    output
}

/// Push a newline, add indention for the next line, and eat all whitespace.
fn newline(indent: i32, iterator: &mut Peekable<Chars>, output: &mut String) {
    output.push('\n');
    for _ in 0..(indent * 2) {
        output.push(' ');
    }
    eat_whitespace(iterator);
}

fn eat_whitespace(iterator: &mut Peekable<Chars>) {
    while let Some(peek) = iterator.peek() {
        if peek.is_whitespace() {
            iterator.next();
        } else {
            break;
        }
    }
}

fn is_open(c: char) -> bool {
    c == '(' || c == '[' || c == '{'
}

fn is_close(c: char) -> bool {
    c == ')' || c == ']' || c == '}'
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_files() {
        let files: Vec<_> = fs::read_dir("examples")
            .unwrap()
            .filter_map(Result::ok)
            .collect();
        let test_files: Vec<_> = files
            .iter()
            .filter(|t| t.path().to_str().unwrap().ends_with(".txt"))
            .collect();
        for file in test_files.iter() {
            let input = fs::read_to_string(file.path()).unwrap();
            let expected_output_path = format!("{}.output", file.path().display());
            let a = std::path::Path::new(&expected_output_path);
            let output = indent(&input);
            if std::env::var("OVERWRITE_TEST_FILES").unwrap_or("false".to_string()) == "true" {
                fs::write(a, &output).unwrap();
            }
            if a.exists() {
                let expected_output = fs::read_to_string(a).unwrap();
                assert_eq!(expected_output, output);
            } else {
                fs::write(a, &output).unwrap();
            }
        }
    }
}