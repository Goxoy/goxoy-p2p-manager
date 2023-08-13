#[allow(non_camel_case_types)]
//#![allow(warnings, unused)]
pub struct p2pManager {
    defined: bool,
}

impl p2pManager {
    pub fn new() -> Self {
        p2pManager {
            defined: false,
        }
    }
}

#[test]
fn full_test() {
    // cargo test  --lib full_test -- --nocapture
    assert!(true)
}
