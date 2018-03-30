
use byteorder::{ByteOrder, LittleEndian};

use scalar::Scalar;

/// Compute w-NAF(x).
/// 
/// The algorithm for computing a w-NAF is described in §3.35 of H-M-V:
/// 
/// ```text
/// naf = [0]
/// while x >= 1:
///     if x is odd:
///         n_i = x mods 2^w
///         naf.append(n_i)
///         x = x - n_i
///     else:
///         n_i = 0
///         naf.append(n_i)
///     x = x // 2
/// # now naf has the naf digits
/// ```
/// Rewrite this as:
/// ```text
/// naf = [0]
/// while x >= 1:
///     if x is even:
///         n_i = 0
///         naf.append(n_i)
///     else:
///         n_i = x mods 2^w
///         naf.append(n_i)
///         x = x - n_i
///     x = x // 2
/// # now naf has the naf digits
/// ```
/// Here `mods` means mod, with signed representatives \\(-2^{w-1},\ldots,0,\ldots,2^{w-1} -1\\).
///
/// Write the bits of x as \\(x_0, ... x_n\\), i.e.,
/// $$
/// x = \sum_{i=0}^{n} x_i 2^i.
/// $$
/// Then 
/// $$
/// x \mod 2^w = \sum_{i=0}^{w-1} x_i 2^i,
/// $$
/// so that 
/// $$
/// x = \sum_{i=0}^{w-1} x_i 2^i + 2^w \sum_{i=0}^{n-w} x_{w+i} 2^i.
/// $$
///
/// If \\(x_0 = 0\\), set `naf[i] = 0`. Otherwise, \\(x\\) is odd.  If
/// $$
/// x \mod 2^w = \sum_{i=0}^{w-1} x_i 2^i < 2^w
/// $$
/// then \\(x \mod 2^w = x \mods 2^w = n_i\\), so
/// $$
/// x - n_i = 2^w \sum_{i=0}^{n-w} x_{w+i} 2^i,
/// $$
/// so
/// ```text
/// naf[0] = window
/// naf[1..w] = 0
/// ```
/// If 
/// $$
/// x \mod 2^w = \sum_{i=0}^{w-1} x_i 2^i \ge 2^w
/// $$
/// then \\(n_i = x \mods 2^w = x \mod 2^w - 2^w \\), so
/// $$
/// x - n_i = 2^w \sum_{i=0}^{n-w} x_{w+i} 2^i + 2^w
/// $$
/// so
/// ```text
/// naf[0] = window - 2^w
/// naf[1..w] = 0
/// carry 1 onto x[w]
/// ```
///
/// Ideally we avoid actually doing a carry, and just keep the carry bit.
fn new_naf(x: &Scalar, w: usize) -> [i8;256] {
    let mut naf = [0i8;256];

    let width = 1 << w;  // 2^w
    let window_mask = width - 1;  // w bits
    
    let mut pos = 0;
    let mut carry = 0;
    let words: [u64;5] = [
        LittleEndian::read_u64(&x.bytes[0..]),
        LittleEndian::read_u64(&x.bytes[8..]),
        LittleEndian::read_u64(&x.bytes[16..]),
        LittleEndian::read_u64(&x.bytes[24..]),
        0 // extra word to avoid extra bounds check in the loop
    ];
    while pos < 256 {
        let u64_pos = pos / 64; // which u64 chunk to read
        let u64_idx = pos % 64; // which bit within that chunk
        let bit_buf = words[u64_pos];
        let mut val = (bit_buf >> u64_idx) & window_mask;
        // if we have some bits in the next word, and there is a next word, 
        // then we should add them to the value
        if u64_idx > (64 - w) {
            let extra_bits = w - (64 - u64_idx); // number of bits are in the next word
            let bit_buf_next = words[u64_pos + 1];
            val += (bit_buf_next & ((1<<extra_bits)-1)) << (w - extra_bits);
        }
        let window = carry + val;

        if window & 1 == 0 {
            pos += 1;
            continue;
        }

        if window < width/2 {
            carry = 0;
            naf[pos] = window as i8;
        } else {
            carry = 1;
            naf[pos] = (window as i8) - (width as i8);
        }
        pos += w;
    }

    naf
}

#[cfg(test)]
mod test {
    use super::*;

    fn naf_compare_helper(x: Scalar) {
        let oldnaf = x.non_adjacent_form().to_vec();
        let newnaf = new_naf(&x, 5).to_vec();
        assert_eq!(oldnaf, newnaf);
    }

    #[test]
    fn compare_newnaf_random() {
        for i in 1..1000 {
            naf_compare_helper(Scalar::from_u64(i).invert());
        }
    }

    use test::Bencher;
    use test::black_box;

    #[bench]
    fn oldnaf(b: &mut Bencher) {
        let x = Scalar::from_u64(9820398).invert();
        b.iter(|| black_box(x.non_adjacent_form()));
    }

    #[bench]
    fn newnaf(b: &mut Bencher) {
        let x = Scalar::from_u64(9820398).invert();
        b.iter(|| black_box(new_naf(&x, 5)));
    }

}

