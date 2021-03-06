fn is_prime(pripos: u32) -> bool {
    for i in 2..(pripos - 1) {
        if pripos % i == 0 {
            return false
        }
    }

    true
}

pub fn nth(n: u32) -> u32 {
    let mut count: u32 = 0;
    let mut i = 2;
    while count != (n + 1) {
     if is_prime(i) {
      count += 1;
     }
    i += 1;
    }
   i - 1
}
