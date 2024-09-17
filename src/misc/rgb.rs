#[derive(Debug)]
pub struct Rgb{
    pub r: u8,
    pub g: u8,
    pub b: u8,
}


impl Rgb{
    pub fn new(r: u8, g:u8, b :u8)-> Self{
        Rgb{r,g,b}
    }
}


pub fn to_rgb(from: &str) -> Rgb
{
    let chars = from.chars();
    let count = chars.count();
    let slice = &from[count-6..];
    let mut bytes: Vec<u8> = vec![];
    let _ = easy_hex::decode(slice, |a|{
        for b in a{
            bytes.push(*b);
        }
    }).unwrap();
    let r = bytes[0];
    let g = bytes[1];
    let b= bytes[2];
    return Rgb{r,g,b};

}



#[cfg(test)]
mod tests {
    use super::*; // Import the functions and types from the parent module

    #[test]
    fn to_rgb_test() {
        let bytes = to_rgb("0xff00ff");
        dbg!(bytes);
    }
}
