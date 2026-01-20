pub fn parts_sums(arr: &[u64]) -> Vec<u64> {
    let mut  k = Vec::new();
 let  mut b = 0;
    for i in arr.iter(){
        k.push(b);
        b+=i;


    } 

    k.push(b);
    k.reverse();
    k
}