/*
 * GPLv3 ( https://opensource.org/licenses/GPL-3.0 )
 */

use bincode::Result;
use serde::de::Deserialize;
use serde::ser::Serialize;

pub trait Binary<'de, T>
    where T: Deserialize<'de>, Self: Serialize
{
    fn from_binary(buf: &'de Vec<u8>) -> Result<T> {
        bincode::deserialize(&buf)
    }

    fn to_binary(&self) -> Result<Vec<u8>> {
        bincode::serialize(&self)
    }
}
