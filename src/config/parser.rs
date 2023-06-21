use crate::config::types::{ Parser, Item };

use nom::{
    character::complete::{alpha1, space0},
    combinator::map_res,
    sequence::{tuple, delimited},
    IResult,
    bytes::complete::{take_until, tag},
};
use tokio::fs::read_to_string;


impl<'lt> Parser<'lt> {
    
    pub async fn parse_input(&mut self) {
        
        self.read_line().await;
    }

    pub fn get_item<'a>(
        &'a mut self,
        input: &'a str
    )-> IResult<&'a str, Item> {
        println!("[LOG::get_item]: {}", input);
       
        let (
            remaining_input,
            (token,_,value)
        ) = tuple((take_until("="), tag("="), take_until("\n")))(input)?;
        println!("Debug - token: {:?}, value: {:?}", token, value);
        Ok((
            remaining_input, 
            Item { 
                token: String::from(token), 
                value: String::from(value) 
            }
        ))
    }

    pub fn debug_item<'a>(
        &'a mut self,
        input: &'a str
    ) -> IResult<&'a str, Item> {
        
        let delimiter_pos = input.find("=");
        match delimiter_pos {
            Some(pos) => {
                let (token, value) = input.split_at(pos);
                let (_, value) = value.split_at(1);
                println!("Debug - Token: {:?}", token.trim());
                println!("Debug - Value: {:?}", value.trim());

                Ok((
                    &input[input.len()..],

                    Item {
                        token: token.trim().to_string(),
                        value: value.trim().to_string()
                    }
                ))
            }
            None => {
                println!("Parsing Error: Delimiter (=) not found");
                Err(nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag)))
            }
        }
    }

    pub async fn read_line(&mut self) {

        let input_string = read_to_string(self.input).await.unwrap();
        let mut tmp_item = Vec::new();

        for line in input_string.lines() {
            let owned_line = line.to_owned();
            let val = self.debug_item(&owned_line);
            match val {
                Ok(l) => { tmp_item.push(l.1)}
                Err(e) => { panic!("{}", e); }
            }
        }

        self.items.extend(tmp_item);
    }
}
