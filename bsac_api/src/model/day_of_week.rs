use serde::{Serialize, Deserialize};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum DayOfWeek {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
}
