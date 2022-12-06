use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let re = regex::Regex::new(r",\s+").unwrap();
    
    input = re.replace_all(&input, ",").to_string();

    let mut output = String::new();
    let mut indent = 0;
    for char in input.chars() {
        if char == '\n' {
            // output.push('\n');
            // push_indent(indent, &mut output);
        } else if char == ',' {
            output.push(char);
            output.push('\n');
            push_indent(indent, &mut output);
        } else if is_open(char) {
            indent += 1;
            output.push(char);
            output.push('\n');
            push_indent(indent, &mut output);
        } else if is_close(char) {
            indent -= 1;
            output.push('\n');
            push_indent(indent, &mut output);
            output.push(char);
        } else {
            output.push(char);
        }
    }
    println!("{output}");
}

fn is_open(c: char) -> bool {
    return c == '(' || c == '[' || c == '{';
}

fn is_close(c: char) -> bool {
    return c == ')' || c == ']' || c == '}';
}

fn push_indent(i: i32, output: &mut String) {
    for _ in 0..(i*2) {
        output.push(' ');
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_input() {
        let input = r"CasingAssembly(minInsideDiameter=Distance(value=20.0, unit=meter), 
        minOutsideDiameter=Distance(value=20.0, unit=meter), 
        maxOutsideDiameter=Distance(value=20.0, unit=meter), 
        originalMeasuredDepthTop=Distance(value=200.0, unit=meter), 
        originalMeasuredDepthBase=Distance(value=300.0, unit=meter), 
        type=null, reportDescription=null, sectionTypeCode=null, 
        cementing=null, components=[CasingComponent(minInsideDiameter=Distance(value=20.0, unit=meter), 
        maxOutsideDiameter=null, topMeasuredDepth=null, baseMeasuredDepth=null, grade=null, 
        connectionName=null, joints=null, description=null, manufacturer=null, typeCode=null, 
        linearWeight=null), CasingComponent(minInsideDiameter=null, maxOutsideDiameter=null, 
            topMeasuredDepth=Distance(value=150.0, unit=meter), baseMeasuredDepth=Distance(value=150.0, 
                unit=meter), grade=null, connectionName=null, joints=null, description=null, 
                manufacturer=null, typeCode=null, linearWeight=null)])";
    }
}
