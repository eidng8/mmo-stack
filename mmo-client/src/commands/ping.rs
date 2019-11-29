/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use chrono::offset::Utc;
use serde::Deserialize;
use serde::Serialize;

use crate::binary::Binary;

#[derive(Serialize, Deserialize)]
pub struct Ping {
    timestamp: i64,
}

impl Ping {
    pub fn new() -> Ping {
        Ping { timestamp: Utc::now().timestamp_millis() }
    }
}

impl<'de> Binary<'de, Ping> for Ping {}
