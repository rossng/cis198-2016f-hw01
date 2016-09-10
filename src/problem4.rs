pub fn bloom(data: &Vec<&str>, hashes: [fn(&[u8]) -> u64; 3], value: &str) -> bool {
    let mut store = [false; 20];

    for s in data {
        for hash in hashes.iter() {
            store[(hash(s.as_bytes()) % 20) as usize] = true;
        }
    }

    hashes.iter().all(|&h| store[(h(value.as_bytes()) % 20) as usize])
}

pub fn djb2(bytes: &[u8]) -> u64 {
    let mut hash: u64 = 5381;
    for b in bytes {
        // hash * 33 + c
        hash = (hash.wrapping_shr(5) + hash) + (*b as u64);
    }

    return hash;
}

pub fn fnv(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf29ce484222325;
    for b in bytes {
        hash = hash ^ (*b as u64);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    return hash;
}

pub fn jenkins(bytes: &[u8]) -> u64 {
    let mut hash = 0;
    for b in bytes {
        hash += *b as u64;
        hash += hash.wrapping_shr(10);
        hash ^= hash.wrapping_shl(6);
    }
    hash += hash.wrapping_shr(3);
    hash ^= hash.wrapping_shl(11);
    hash += hash.wrapping_shr(15);
    return hash;
}
