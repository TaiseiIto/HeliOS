use crate::rs232c;

#[allow(dead_code)]
#[derive(Debug)]
pub struct Argument<'a> {
    com: &'a mut rs232c::Com,
}

impl<'a> Argument<'a> {
    pub fn new(com: &'a mut rs232c::Com) -> Self {
        Self {
            com,
        }
    }
}

