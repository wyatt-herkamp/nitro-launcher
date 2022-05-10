pub mod microsoft;
pub mod offline;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum Authentication {
    Microsoft,
    Offline,
}