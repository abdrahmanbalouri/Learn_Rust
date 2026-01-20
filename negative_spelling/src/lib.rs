pub fn negative_spell(mut n: i64) -> String {
    if n > 0 {
        return "error: positive number".to_string();
    }
    if n == 0 {
        return "zero".to_string();
    }

    let c = String::from("minus ");
    n = n * -1;
    fn spell(n: i64) -> String {
        let mut c = String::new();
        let ones = vec![
            "", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
            "eleven", "twelve", "thirten", "fourteen", "fiveten", "sixten", "seventen", "eighten",
            "nineten",
        ];
        let tens = vec![
            "", "", "twenty", "thirty", "fourty", "fifty", "sixty", "seventy", "eighty", "nenty",
        ];
        if n < 20 {
            c.push_str(ones[n as usize]);
        } else if n < 100 {
            c.push_str(tens[(n / 10) as usize]);
            if n % 10 != 0 {
                c.push_str("-");
            }
            c.push_str(ones[(n % 10) as usize]);
        } else if n < 1000 {
            c.push_str(ones[(n / 100) as usize]);
            c.push_str(" hundred");
            if n % 100 != 0 {
                c.push(' ');
            }
            c.push_str(&spell(n % 100));
        } else if n < 1000000 {
            c.push_str(&spell(n / 1000));
            c.push_str(" thousand");
            if n % 1000 != 0 {
                c.push(' ');
            }
            c.push_str(&spell(n % 1000));
        } else if n < 1000000000 {
            c.push_str(&spell(n / 1000000));
            c.push_str(" million");
            if n % 1000000 != 0 {
                c.push(' ');
            }
            c.push_str(&spell(n % 1000000));
        } else if n < 1000000000000 {
            c.push_str(&spell(n / 1000000000));
            c.push_str(" billions");
            if n % 1000000000 != 0 {
                c.push(' ');
            }
            c.push_str(&spell(n % 1000000000));
        }

        c
    }

    let k = spell(n);

    return c + &k;
}
