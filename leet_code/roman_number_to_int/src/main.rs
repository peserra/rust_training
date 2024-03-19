use std::collections::HashMap;
fn main(){
    let int_num = roman_to_int(String::from("LVIII"));
    println!("{int_num}");
}

fn roman_to_int(s: String) -> i32 {
    let letters_map = HashMap::from
    ([
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000)
    ]);

    /*
        subtrai valor se vier antes: 
        I pode ir antes de V e X
        X pode ir antes de L e C
        C pode ir antes de D e M
    */
    let mut integer_num = 0;
    let mut previous_char = 'Z';
    for romam_num in s.chars().rev(){
        match letters_map.get(&romam_num){
            Some(value) => {
                if romam_num == 'C' && (previous_char == 'D' || previous_char == 'M'){
                    let c_value = letters_map[&'C']; // versao mais unsafe
                    integer_num -= c_value;
                }
                else if  romam_num == 'X' && (previous_char == 'L' || previous_char == 'C'){
                    let x_value:Option<&i32> = letters_map.get(&'X'); // versao mais safe
                    integer_num -= x_value.unwrap();
                }
                else if  romam_num == 'I' && (previous_char == 'V' || previous_char == 'X'){
                    let i_value:Option<&i32> = letters_map.get(&'I');
                    integer_num -= i_value.unwrap();
                }
                else {
                    integer_num += value;
                }
            }
            None => {
                
            }
        }
        previous_char = romam_num;
    }

    return integer_num;
}