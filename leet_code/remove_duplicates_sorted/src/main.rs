
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    nums.dedup();
    nums.len() as i32
}


fn main() {
    let mut v1 = vec![1,1,2];
    let mut v2 = vec![0,0,1,1,1,2,2,3,3,4];
    let r1:i32 = remove_duplicates(&mut v1); 
    let r2:i32 = remove_duplicates(&mut v2);

    println!("{r1} e {r2}"); 

}
