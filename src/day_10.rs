use crate::helpers::read_lines;

pub fn day_10_part_1() {
    if let Ok(file_lines) = read_lines("resources/day_10.txt") {
        let mut illegal_score = 0;

        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut char_stack = Vec::<char>::new();

            for c in line.chars() {
                if c == '(' {
                    char_stack.push('(');
                } else if c == ')' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '(' {
                        illegal_score += 3;
                        break;
                    }
                } else if c == '[' {
                    char_stack.push('[');
                } else if c == ']' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '[' {
                        illegal_score += 57;
                        break;
                    }
                } else if c == '{' {
                    char_stack.push('{');
                } else if c == '}' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '{' {
                        illegal_score += 1197;
                        break;
                    }
                } else if c == '<' {
                    char_stack.push('<');
                } else if c == '>' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '<' {
                        illegal_score += 25137;
                        break;
                    }
                }
            }
        }

        println!("Total syntax error score = {}", illegal_score);
    }
}

pub fn day_10_part_2() {
    if let Ok(file_lines) = read_lines("resources/day_10.txt") {
        let mut completion_scores = Vec::<u64>::new();

        for file_line in file_lines {
            let line = file_line.unwrap();

            let mut char_stack = Vec::<char>::new();

            let mut corrupt = false;

            for c in line.chars() {
                if c == '(' {
                    char_stack.push('(');
                } else if c == ')' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '(' {
                        corrupt = true;
                        break;
                    }
                } else if c == '[' {
                    char_stack.push('[');
                } else if c == ']' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '[' {
                        corrupt = true;
                        break;
                    }
                } else if c == '{' {
                    char_stack.push('{');
                } else if c == '}' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '{' {
                        corrupt = true;
                        break;
                    }
                } else if c == '<' {
                    char_stack.push('<');
                } else if c == '>' {
                    if char_stack.len() == 0 || char_stack.pop().unwrap() != '<' {
                        corrupt = true;
                        break;
                    }
                }
            }

            if !corrupt {
                let mut completion_score: u64 = 0;
    
                while char_stack.len() > 0 {
                    completion_score *= 5;
    
                    match char_stack.pop().unwrap() {
                        '(' => completion_score += 1,
                        '[' => completion_score += 2,
                        '{' => completion_score += 3,
                        '<' => completion_score += 4,
                        _ => ()
                    }
                }
    
                completion_scores.push(completion_score);
            }
        }

        completion_scores.sort();

        println!("Total syntax error score = {}", completion_scores.get(completion_scores.len()/2).unwrap());
    }
}
