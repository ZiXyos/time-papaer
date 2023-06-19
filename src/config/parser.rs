use crate::config::types::{ Parser, Value, Entry };

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while},
    combinator::{map_res, opt},
};

use nom_locate::LocatedSpan;

impl<'lt> Parser<'lt>{
    
    pub fn parse_input(&mut self) {

    }
}
