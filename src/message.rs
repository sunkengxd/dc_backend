use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Deserialize, Validate)]
pub struct Message {
    #[validate(range(min = 0, max = 5))]
    pub speed: i32,
    pub is_forward: bool,
}
