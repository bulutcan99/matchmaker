use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum Position {
    CEO,
    Manager,
    WhiteCollar,
    BlueCollar,
}
