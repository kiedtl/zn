use std::vec::Vec;

pub fn lex(contents: String) -> Vec<String> {
    let mut commands: Vec<String> = Vec::new();
    let mut buf = "".to_owned();

    let lines = contents.split("\n").collect::<Vec<&str>>();

    for mut i in 0..lines.len() {
        let line = lines[i];
        let chars = line.chars().collect::<Vec<char>>();
        for mut c in 0..chars.len() {
            // comments
            if chars[c] == '\"' {
                i = i + 1;
                break;
            } else if is_command(chars[c]) {
                buf.push(chars[c]);
                c = c + 1;
                if c == chars.len() {
                    break;
                }
                while !is_command(chars[c]) {
                    buf.push(chars[c]);
                    c = c + 1;
                    if c == chars.len() {
                        break;
                    }
                }
                commands.push(buf);
                buf = "".to_owned();
            }
        }
    }
    commands
}

fn is_command(c: char) -> bool {
    match c {
        '$'|
            '#'|
            '^'|
            '*'|
            '&'|
            '-'|
            '+'|
            '|'|
            ':'|
            ';'|
            '<'|
            '>'|
            '.'|
            ','|
            '?'|
            'ø'|
            'ß' => true,
            _ => false,
    }
}
