use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum WorkTypeEnum {
    Laboratory,
    Practical,
    Okr,
    DifferentiatedTest,
}
