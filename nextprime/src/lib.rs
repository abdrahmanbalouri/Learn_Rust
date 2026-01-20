pub fn next_prime(nbr: usize) -> usize {
    if nbr==0{
        return 2;
    }
    
    for i in nbr..10000000 {
        if isprime(i) {
            return i;
        }
    }
    return nbr;
}

fn isprime(nb: usize) -> bool {
    for i in 3..nb {
        if nb % i == 0 {
            return false;
        }
    }
    return true;
}
