pub enum Ty {
    A(i32),
    B(String),
}

impl From<i32> for Ty {
    fn from(s: i32) -> Self {
        Self::A(s)
    }
}

impl From<String> for Ty {
    fn from(s: String) -> Self {
        Self::B(s)
    }
}

#[allow(dead_code)]
pub fn set(_: impl Into<Ty>) {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        set(String::from("dslkdslk"));
        set(4343);
    }
}

pub mod ws;