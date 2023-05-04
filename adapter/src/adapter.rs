use crate::{adaptee::SpecificTarget, Target};

pub struct TargetAdapter {
    adaptee: SpecificTarget,
}

impl TargetAdapter {
    pub fn new(adaptee: SpecificTarget) -> Self {
        Self { adaptee }
    }
}

// 適合するために SpecificTarget の request を逆順にして返す
impl Target for TargetAdapter {
    fn request(&self) -> String {
        self.adaptee.specific_request().chars().rev().collect()
    }
}
