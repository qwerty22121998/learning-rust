const MAX_N: u32 = 1e7 as u32;

pub fn eratos(n: u32) -> Vec<u32> {
    let mut is_prime = vec![true; n as usize];
    let mut result: Vec<u32> = Vec::new();
    is_prime[0] = false;
    is_prime[1] = false;
    for i in 2..=(n as f32).sqrt() as usize {
        if !is_prime[i] {
            continue;
        }
        let mut j = i * i;
        while j < n as usize {
            is_prime[j] = false;
            j += i;
        }
    }

    for i in 2..n {
        if is_prime[i as usize] {
            result.push(i)
        }
    }

    result
}

pub fn nth(n: u32) -> u32 {
    let prime = eratos(MAX_N);
    prime[n as usize]
}
