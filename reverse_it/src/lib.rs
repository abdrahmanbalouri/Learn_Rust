pub fn reverse_it(mut v: i32) -> String {
    let mut k = false;
    if v < 0 {
        v = v * -1;
        k =true;
    }

    let c = v.to_string();
    let p : String= c.chars().rev().collect();

    if k{
      return "-".to_string() + &p +&c;
    }

          return  p +&c;

}
