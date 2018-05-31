use std::fmt;
use std::env;
use std::ops;

// newtype defined here
struct StrWrapper(Vec<String>);

// implementation of the printing trait for the newtype
impl fmt::Display for StrWrapper {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0.join(", "))
    }
}

// implementation of adding trait (i.e. string concatenation)
impl ops::Add for StrWrapper {
    type Output = StrWrapper;
    fn add(self, other: StrWrapper) -> StrWrapper {
        let mut r = Vec::new();
        r.extend_from_slice(&self.0[..]);
        r.extend_from_slice(&other.0[..]);
        StrWrapper(r)
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let s = StrWrapper(args[1..].to_vec());
    let t = StrWrapper(vec!(String::from("Rust")));
    println!("{}", s + t);
}