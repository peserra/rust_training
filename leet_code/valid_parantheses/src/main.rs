fn main() {
is_valid(String::from("){"));
}

pub fn is_valid(s: String) -> bool {
   let mut bracket_stack:Vec<char> = vec![];
    for c in s.chars(){
        if s.len() < 2 {
            return false;
        }
        match c {
            '(' | '[' | '{' => bracket_stack.push(c), 
            ')' => {
                if bracket_stack.last() == None {
                    return false;
                }
                if bracket_stack.last().unwrap() != &'('{return false;}
                bracket_stack.pop();
            },
            ']' => {
                if bracket_stack.last() == None 
                {
                    return false;
                }
                if bracket_stack.last().unwrap() != &'[' {return false;}
                bracket_stack.pop();
            },
            '}' => {
                if bracket_stack.last() == None {
                    return false;
                }
                if bracket_stack.last().unwrap() != &'{' {return false;}
                bracket_stack.pop();
            }
            _ => return  false
        }
    }

    return bracket_stack.is_empty();
}