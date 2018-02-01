use std::fmt;

#[derive(Copy, Clone)]
pub struct RGBA<A> {
    pub red: A,
    pub green: A,
    pub blue: A,
    pub alpha: A
}

impl<A: fmt::Display> fmt::Display for RGBA<A> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(RGBA {{ red: {}, green: {}, blue: {}, alpha: {} }})", self.red, self.green, self.blue, self.alpha)
    }
}

impl<A> RGBA<A> {
    pub fn of(red: A, green: A, blue: A, alpha: A) -> RGBA<A> {
        RGBA {
            red: red,
            green: green,
            blue: blue,
            alpha: alpha
        }
    }

    pub fn ofRGBA(red: A, green: A, blue: A, alpha: A) -> RGBA<A> {
        RGBA::of(red, green, blue, alpha)
    }
}

impl RGBA<u8> {
    pub fn ofRGB(red: u8, green: u8, blue: u8) -> RGBA<u8> {
        RGBA::of(red, green, blue, 255u8)
    }
}
