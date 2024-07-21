use super::common::Algorithm;

pub struct MockAlgorithm {}

impl Algorithm for MockAlgorithm {
    fn decision(_state: &crate::state::RlStateInternal) -> Vec<super::common::Choice> {
        Vec::new()
    }
}
