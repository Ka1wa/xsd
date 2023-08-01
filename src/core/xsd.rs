#[derive(Clone, Debug)]
pub struct Xsd {
    left: i32,
    right: i32
}

impl Default for Xsd {
    fn default() -> Self {
        Self {
            left: 0,
            right: 0,
        }
    }
}

impl Xsd {
    pub fn new(left: i32, right: i32) -> Xsd {
        Self {
            left,
            right
        }
    }

    pub fn add(self) -> i32 {
        self.left + self.right
    }
}
