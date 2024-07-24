pub fn hailstone(n: u64) -> u64 {
    // TODO
    if n % 2 == 0 {
        n / 2
    } else {
        3 * n + 1
    }
}

// This function's average time for 7 is 299.69 ns, for 162964 is 565.07 ns, and for 686901248 is 3.4182 µs
pub fn hailstone_sequence_append(n: u64) -> Vec<u64> {
    // TODO
    let mut sequence = Vec::new();
    let mut current = n;
    
    sequence.push(current);
    
    while current != 1 {
        current = hailstone(current);
        sequence.push(current);
    }
    
    sequence
}

// The average time for 7 is 71.440 ns, for 162964 is 457.31 ns, and for 686901248 is 1.8880 µs
pub fn hailstone_sequence_prealloc(n: u64) -> Vec<u64> {
    // TODO
    let length = hailstone_length(n);
    let mut sequence = Vec::with_capacity(length);
    let mut current = n;
    
    sequence.push(current);
    
    while current != 1 {
        current = hailstone(current);
        sequence.push(current);
    }
    
    sequence
}

// Helper function finds length of sequence in order to pre-allocate
fn hailstone_length(n: u64) -> usize {
    let mut length = 1;
    let mut current = n;
    
    while current != 1 {
        current = hailstone(current);
        length += 1;
    }
    
    length
}
