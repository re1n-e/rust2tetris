use crate::gates::*;

// HalfAdder
pub fn half_adder(a: u8, b: u8) -> (u8, u8) {
    let sum: u8 = xor(a, b);        // Sum is XOR of a and b
    let carry_out: u8 = and(a, b);  // Carry is AND of a and b
    (sum, carry_out)
}

// FullAdder
pub fn full_adder(a: u8, b: u8, cin: u8) -> (u8, u8) {
    let (sum1, carry1): (u8, u8) = half_adder(a, b);  // First half-adder for a and b
    let (sum, carry2): (u8, u8) = half_adder(sum1, cin);  // Second half-adder for sum1 and carry-in
    let carry_out: u8 = or(carry1, carry2);  // Final carry-out is OR of the two carries
    (sum, carry_out)
}

// Add16 16-bit Adder chip is used to add two 16-bit numbers.
pub fn add_16(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let mut res: [u8; 16] = [0u8; 16];
    let mut carry: u8;

    // First addition using the half-adder
    let (sum, carry_out) = half_adder(a[0], b[0]);
    res[0] = sum;
    carry = carry_out;
    // Full adder for the remaining bits
    for i in 1..16 {
        let (sum, carry_out) = full_adder(a[i], b[i], carry);
        res[i] = sum;
        carry = carry_out;
    }
    res
}

// Inc16
pub fn inc_16(a: [u8; 16]) -> [u8; 16] {
    let mut b: [u8; 16] = [0u8; 16];
    b[0] = 1;  // Increment the last bit
    add_16(a, b)
}

// (vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0] ,vec![1,1,1,1,1,1,1,1,1,1,1,1,1,1,1,1] ,1, 1, 1, 1, 1, 1, vec![0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,1] ,0, 0),
// ALU function with zx, nx, zy, ny, f, no, and additional output flags
pub fn alu(x: [u8; 16], y: [u8; 16], zx: u8, nx: u8, zy: u8, ny: u8, f: u8, no: u8) -> ([u8; 16], u8, u8) {
    // Process x input based on zx and nx
    let x_processed = match (zx, nx) {
        (0, 0) => x,
        (1, 1) => not_16([0u8; 16]),
        (1, 0) => [0u8; 16], // Zero x
        (0, 1) => not_16(x), // Negate x
        _ => unreachable!(), // Should never happen
    };
    
    // Process y input based on zy and ny
    let y_processed = match (zy, ny) {
        (0, 0) => y,
        (1, 1) => not_16([0u8; 16]),
        (1, 0) => [0u8; 16], // Zero y
        (0, 1) => not_16(y), // Negate y
        _ => unreachable!(), // Should never happen
    };

    // Perform operation based on f
    let result = match f {
        0 => and_16(x_processed, y_processed), // AND
        1 => add_16(x_processed, y_processed), // ADD
        _ => unreachable!(), // Should never happen
    };
    for i in 0..16 {
        print!("{} ", result[i]);
    }

    // Negate result if no is 1
    let out = if no == 1 { not_16(result) } else { result };

    // Calculate zr (zero flag)
    let zr = out.iter().all(|b| *b == 0);

    // Calculate ng (negative flag)
    let ng = out[15] == 1;

    (out, zr as u8, ng as u8)
}

