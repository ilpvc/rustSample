#[cfg(test)]
mod tests {
    use crate::codec::base16_codec::encode;

    #[test]
    fn test1(){
        let percent = "%%%%你好";
        let vec = encode(percent.as_bytes());
        vec.iter().for_each(|e|{
            print!("{}",e)
        })
    }
}