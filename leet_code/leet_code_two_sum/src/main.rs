use std::collections::HashMap;
fn main() {
    let numbers:Vec<i32> = vec![1,2,3,4];
    let target = 9;
    two_sum(numbers, target);
}


fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32>{
   let mut seen = HashMap::with_capacity(nums.len());

   // guardando valores e indices ja visto
   for(i, &num) in nums.iter().enumerate() {
        // verifica se há diferença do valor target - atual  (target - num) no hashmap 
        match seen.get(&num){
            // se nao for encontrado tal valor, adiciona ao hashmap
            None => {seen.insert(target - num, i);},
            // se foi encontrado, achou a soma e retorna value do dicionario que corresponde à chave(&j) do numero achado e o indice i do numero atual 
            Some(&j) => return vec![j as i32, i as i32], 
        }
   }
   unreachable!()
}
