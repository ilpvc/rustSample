#[cfg(test)]
mod tests {
    use crate::codec::base16_codec::{decode, encode};

    #[test]
    fn test1(){
        let percent = "*&^";
        let vec = encode(percent.as_bytes());
        vec.iter().for_each(|e|{
            print!("{}",e)
        });
        println!("\n");
        let option = decode(vec.as_slice());
        match option {
            Some(v)=>v.iter().for_each(|x|print!("{}",x)),
            None=>print!("null")
        }
    }
}