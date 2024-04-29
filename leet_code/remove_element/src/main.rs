
// remove todas as ocorrencias de val no array, inplace
// retorna numero de elementos do array que nao sao val
pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
   let mut isn_indx = 0;
   for i in 0..nums.len(){
        if nums[i] != val{
            nums[isn_indx] = nums[i];
            isn_indx += 1
        }
   }
   return isn_indx as i32;
}


fn main() {

    let mut v1 = vec![3,2,2,3];
    let k = remove_element(&mut v1, 3);
    println!("{k}");
}
