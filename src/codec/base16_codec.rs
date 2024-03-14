

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

pub fn decode(data: &[char]) -> Option<Vec<u8>>{
    if data.len()%2==1 {
        return None
    }
    let mut de_data = Vec::with_capacity(data.len()/2);
    for chunk in data.chunks(2) {
        let high_nibble = match chunk[0] {
            '0'..='9' => chunk[0] as u8 - b'0',
            'A'..='F' => chunk[0] as u8 - b'A' + 10,
            'a'..='f' => chunk[0] as u8 - b'a' + 10,
            _ => return None, // 如果遇到非十六进制字符，则无法进行解码
        };

        let low_nibble = match chunk[1] {
            '0'..='9' => chunk[1] as u8 - b'0',
            'A'..='F' => chunk[1] as u8 - b'A' + 10,
            'a'..='f' => chunk[1] as u8 - b'a' + 10,
            _ => return None, // 如果遇到非十六进制字符，则无法进行解码
        };

        let byte = (high_nibble << 4) | low_nibble;

        de_data.push(byte)
    }
    Some(de_data)
}