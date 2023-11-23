use crate::*;


#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum CborContainerType {
    Array = 0,
    Map = 1,
}