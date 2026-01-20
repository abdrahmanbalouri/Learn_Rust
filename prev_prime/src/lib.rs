pub fn prev_prime(nbr: u64) -> u64 {
    for i in (2..nbr).rev() {
        if isprime(i) {
            return i;
        }
    }
    return 0;
}

fn isprime(nb: u64) -> bool {
    for i in 2..nb {
        if nb % i == 0 {
            return false;
        }
    }
    return true;
}
