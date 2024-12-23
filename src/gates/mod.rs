// Nand 
// pub fn nand(a: u8, b: u8) -> u8 {
//     if a & b == 1 {
//         0
//     } else {
//         1
//     }
// }
pub fn nand(a: u8, b: u8) -> u8 {
    (!(a & b)) & 1
}

pub fn not(a: u8) -> u8 {
    nand(a, a)
}

pub fn and(a: u8, b: u8) -> u8 {
    not(nand(a,b))
}

pub fn or(a: u8, b: u8) -> u8 {
    nand(not(a), not(b))
}

pub fn xor(a: u8, b: u8) -> u8 {
    or(and(a, not(b)), and(not(a), b))
    // nand(nand(a, b), nand(not(a), not(b)))
}

pub fn mux(a: u8, b: u8, sel: u8) -> u8 {
    or(and(not(sel),a),and(sel, b))
}

pub fn d_mux(inp: u8, sel: u8) -> (u8, u8) {
    (and(not(sel), inp), and(sel, inp))
}

pub fn not_16(inp: [u8; 16]) -> [u8; 16] {
    let mut res: [u8; 16] = [0u8; 16];
    for i in 0..16 {
        res[i] = not(inp[i]);
    }
    res
}

// And16
pub fn and_16(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let mut res: [u8; 16] = [0u8; 16];
    for i in 0..16 {
        res[i] = and(a[i], b[i]);
    }
    res
}

// Or16
pub fn or_16(a: [u8; 16], b: [u8; 16]) -> [u8; 16] {
    let mut res: [u8; 16] = [0u8; 16];
    for i in 0..16 {
        res[i] = or(a[i], b[i]);
    }
    res
}

// Mux16
pub fn mux_16(a: [u8; 16], b: [u8; 16], sel: u8) -> [u8; 16] {
    let mut res: [u8; 16] = [0u8; 16];
    for i in 0..16 {
        res[i] = mux(a[i], b[i], sel);
    }
    res
}

pub fn or_8way(inp: [u8; 8]) -> u8 {
    inp.iter().fold(0, |acc, &byte| or(acc, byte))
}

pub fn mux_4way16(a: [u8; 16], b: [u8; 16], c: [u8; 16], d: [u8; 16], sel: [u8; 2]) -> [u8; 16] {
    let ab = mux_16(a, b, sel[0]);
    let cd = mux_16(c, d, sel[0]);
    mux_16(ab, cd, sel[1])
}

pub fn mux_8way16(a: [u8; 16], b: [u8; 16], c: [u8; 16], d: [u8; 16], e: [u8; 16], f: [u8; 16], g: [u8; 16], h: [u8; 16], sel: [u8; 3]) -> [u8; 16] {
    let abcd = mux_4way16(a, b, c, d, [sel[0], sel[1]]);
    let efgh = mux_4way16(e, f, g, h, [sel[0], sel[1]]);
    mux_16(abcd, efgh, sel[2])
}

pub fn dmux_4way(inp: u8, sel: [u8; 2]) -> (u8, u8, u8, u8) {
    let (ab, cd) = d_mux(inp, sel[1]);
    let (a, b) = d_mux(ab, sel[0]);
    let (c, d) = d_mux(cd, sel[0]);
    (a, b, c, d)
}

pub fn dmux_8way(inp: u8, sel: [u8; 3]) -> (u8, u8, u8, u8, u8, u8, u8, u8) {
    let (ae, bf, cg, dh) = dmux_4way(inp, [sel[0], sel[1]]);
    let (a , e) = d_mux(ae, sel[2]);
    let (b , f) = d_mux(bf, sel[2]);
    let (c , g) = d_mux(cg, sel[2]);
    let (d , h) = d_mux(dh, sel[2]);
    (a, b, c, d, e, f, g, h)
}
