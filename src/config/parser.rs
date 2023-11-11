use crate::config::types::{Item, Parser};

use nom::IResult;
use tokio::fs::read_to_string;

impl<'lt> Parser<'lt> {
    pub async fn parse_input(&mut self) {
        self.read_line().await;
    }

    fn get_item<'a>(&'a mut self, input: &'a str) -> IResult<&'a str, Item> {
        let delimiter_pos = input.find("=");
        match delimiter_pos {
            Some(pos) => {
                let (token, value) = input.split_at(pos);
                let (_, value) = value.split_at(1);

                Ok((
                    &input[input.len()..],
                    Item {
                        token: token.trim().to_string(),
                        value: value.trim().to_string(),
                    },
                ))
            }
            None => {
                println!("Parsing Error: Delimiter (=) not found");
                Err(nom::Err::Error(nom::error::Error::new(
                    input,
                    nom::error::ErrorKind::Tag,
                )))
            }
        }
    }

    async fn read_line(&mut self) {
        let input_string = read_to_string(self.input).await.unwrap();
        let mut tmp_item = Vec::new();

        for line in input_string.lines() {
            let owned_line = line.to_owned();
            let val = self.get_item(&owned_line);
            match val {
                Ok(l) => tmp_item.push(l.1),
                Err(e) => {
                    panic!("WOOOWWWW {}", e);
                }
            }
        }

        self.items.extend(tmp_item);
        println!("{:?}", self.items);
    }
}
