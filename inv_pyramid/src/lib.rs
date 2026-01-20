pub fn inv_pyramid(v: String, i: usize) -> Vec<String> {
    if i ==0 {
        return Vec::new();
    }
    let mut k = Vec::new();

    for s in 1..=i {
        let mut d = String::new();

        d.push_str(&" ".repeat(s));
        d.push_str(&v.repeat(s));
        k.push(d);
    }
    let mut f = k.clone();
     f.pop();
     f.reverse();

    k.extend(f);
    k
}
