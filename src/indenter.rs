use nom::{
    branch::alt,
    bytes::complete::{tag, take_while, take_while_m_n},
    character::complete::{anychar, char, none_of, one_of},
    combinator::eof,
    multi::{many0, many1},
    sequence::preceded,
    sequence::terminated,
    IResult,
};

pub fn indent(input: &str, indent_size: u8) -> String {
    let input = input.to_string();

    let mut output = String::new();
    let mut indent = 0;

    let mut chars = input.as_str();
    while !chars.is_empty() {
        if let Some((i, ind)) = next(chars, indent, &mut output, indent_size) {
            chars = i;
            indent = ind;
        } else if let Ok((i, c)) = anychar::<&str, ()>(chars) {
            output.push(c);
            chars = i;
        }
    }

    output
}

fn next<'a>(
    i: &'a str,
    indent: i32,
    output: &mut String,
    indent_size: u8,
) -> Option<(&'a str, i32)> {
    let mut indent = indent;
    if let Ok((i, s)) = terminated(empty_parens, space)(i) {
        output.push_str(&s);
        Some((i, indent))
    } else if let Ok((i, c)) = terminated(lpar, space)(i) {
        indent += 1;
        output.push(c);
        newline(indent, output, indent_size);
        Some((i, indent))
    } else if let Ok((i, c)) = terminated(rpar, space)(i) {
        indent -= 1;
        newline(indent, output, indent_size);
        output.push(c);
        Some((i, indent))
    } else if let Ok((i, _)) = terminated(comma, space)(i) {
        output.push(',');
        // Only output a newline if this is a trailing comma
        if !is_end_of_block(i) {
            newline(indent, output, indent_size);
        }
        Some((i, indent))
    } else if let Ok((i, c)) = terminated(string, space)(i) {
        output.push_str(&c);
        Some((i, indent))
    } else if let Ok((i, _)) = terminated(many1(char('\n')), space)(i) {
        if !is_end_of_block(i) {
            newline(indent, output, indent_size);
        }
        Some((i, indent))
    } else {
        None
    }
}

fn is_end_of_block(i: &str) -> bool {
    eof::<&str, ()>(i).is_ok() || rpar(i).is_ok()
}

fn comma(i: &str) -> IResult<&str, ()> {
    let (i, _) = char(',')(i)?;
    let (i, _) = space(i)?;
    Ok((i, ()))
}

fn space(i: &str) -> IResult<&str, &str> {
    let chars = " \t\r\n";
    take_while(move |c| chars.contains(c))(i)
}

fn empty_parens(i: &str) -> IResult<&str, String> {
    let (i, start) = terminated(lpar, space)(i)?;
    let (i, end) = rpar(i)?;
    let mut s = String::new();
    s.push(start);
    s.push(end);
    Ok((i, s))
}

fn lpar(i: &str) -> IResult<&str, char> {
    one_of("([{")(i)
}

fn rpar(i: &str) -> IResult<&str, char> {
    one_of(")]}")(i)
}

fn newline(indent: i32, output: &mut String, indent_size: u8) {
    output.push('\n');
    for _ in 0..(indent * indent_size as i32) {
        output.push(' ');
    }
}

fn string_delimiter(i: &str) -> IResult<&str, char> {
    char('"')(i)
}

fn string(i: &str) -> IResult<&str, String> {
    let mut s = String::new();
    let (i, c1) = string_delimiter(i)?;
    s.push(c1);
    let (i, inner) = parse_str(i)?;
    s.push_str(&inner);
    let (i, c2) = string_delimiter(i)?;
    s.push(c2);
    Ok((i, s))
}

fn escaped(i: &str) -> IResult<&str, String> {
    let (i, _) = char('\\')(i)?;
    match one_of("nt\"\\")(i) {
        Ok((rest, c)) => match c {
            'n' => Ok((rest, "\\n".to_string())),
            't' => Ok((rest, "\\t".to_string())),
            '\'' => Ok((rest, "\\'".to_string())),
            '\"' => Ok((rest, "\\\"".to_string())),
            _ => panic!("Didn't now how to handle {}", c),
        },
        Err(e) => Err(e),
    }
}

fn parse_str(i: &str) -> IResult<&str, String> {
    let any_string_char = |i| match none_of("\"")(i) {
        Ok((rest, c)) => Ok((rest, c.to_string())),
        Err(x) => Err(x),
    };
    let newline_in_string = |i| match char('\n')(i) {
        Ok((rest, _)) => Ok((rest, "\\n".to_string())),
        Err(x) => Err(x),
    };
    let string_path = alt((unicode_letter, escaped, newline_in_string, any_string_char));
    let (i, parts) = many0(string_path)(i)?;
    let joined = parts.join("");
    Ok((i, joined))
}

fn unicode_letter(i: &str) -> IResult<&str, String> {
    let (rest, digits) = preceded(tag("\\u"), four_digits)(i)?;
    let num = u32::from_str_radix(digits, 16).expect("Couldn't parse str radix");
    let c = std::char::from_u32(num).expect("Couldn't create char from parsed str radix");
    Ok((rest, c.to_string()))
}

fn four_digits(i: &str) -> IResult<&str, &str> {
    take_while_m_n(4, 4, |c: char| c.is_ascii_hexdigit())(i)
}

#[cfg(test)]
mod test {
    use std::fs;

    use super::*;
    use pretty_assertions::assert_eq;

    use indoc::indoc;

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
            let output = indent(&input, 2);
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

    #[test]
    fn trailing_comma_no_newline() {
        let input = indoc! {r#"{
            "a": [
              23,
            ]
          }"#};
        let output = indent(input, 2);
        assert_eq!(input, output);
    }
}
