pub mod entitys;
pub mod dto;
pub mod request;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug,Deserialize,Serialize,PartialEq,Eq)]
pub enum GotStatusType {
    Not,
    Yet,
    Lost,
    Cannot
}

#[derive(Clone, Debug,Deserialize,Serialize,PartialEq,Eq)]
pub enum NetName {
    Youtube,
    Videvo,
    MixKit,
    Pixabay,
    Pexel,
}