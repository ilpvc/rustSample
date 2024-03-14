

pub fn encode(data: &[u8]) -> Vec<char>{
    let len = data.len();
    let alphabets:[char; 16] = ['0','1','2','3','4','5','6','7','8','9','A','B','C','D','E','F'];
    let mut out = Vec::with_capacity(len << 1);

    for &byte in data {
        let high_nibble = (0xF0 & byte) >> 4;
        let low_nibble = 0x0F & byte;

        out.push(alphabets[high_nibble as usize]);
        out.push(alphabets[low_nibble as usize]);
    }
    out
}