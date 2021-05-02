//! caesar cipher

pub fn caesar(cipher: &str, shift:u8)-> String{
    cipher
        .chars()
        .map(|c| {
            //a-z A-Z에 속하는지 확인
            if c.is_ascii_alphabetic(){
                //a-z이면 b'a', A-Z이면 b'A'
                let first = if c.is_ascii_lowercase() {b'a'} else {b'A'};
                // module the distance to keep character range
                (first + (c as u8 + shift - first) % 26 )as char
            }else{
                c
            }
        })
        .collect()
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn empty(){
        assert_eq!(caesar("",13), "");
    }

    #[test]
    fn caesar_rot_13(){
        assert_eq!(caesar("rust",13), "ehfg");
    }
    #[test]
    fn caesar_unicode(){
        assert_eq!(caesar("attack at dawn 攻",15), "fyyfhp fy ifbs 攻");
    }
}