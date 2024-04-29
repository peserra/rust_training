
fn main() {
    let number:i32 = -121;
    let result = is_palindrome(number);
    println!("{result}"); 
}


pub fn is_palindrome(x: i32) -> bool {
    let mut dig = Vec::new();
    let mut number:i32 = x;
    let mut r:Option<&i32>;
    let mut l:Option<&i32>;
   
    if number.is_negative() {
        return false;
    }

    // push digits into array
    while number > 9 {
        dig.push(number % 10);
        number = number / 10;
    }
    dig.push(number);
    let dig_length = dig.len();

    for i in 0..dig_length/2 {
        l = dig.first();
        r = dig.last();
        if l.unwrap() != r.unwrap() {
            return false
        }
        dig.remove(0);
        dig.pop();
    }
    return true;   
}