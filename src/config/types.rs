
#[derive(Debug)]
pub enum Value {
   
   String(String),
   Vec(Vec<String>),
   Bool(bool),
   Integer(i32),
   EOF
}

#[derive(Debug)]
pub struct Entry {

   pub key: String,
   pub value: Value
}

#[derive(Debug, Clone, Copy)]
pub struct Setting {

   pub weather: bool,
}

#[derive(Debug)]
pub struct Parser<'lt> {

   pub input: &'lt str,
   pub items: Vec<Item>,
}

#[derive(Debug)]
pub struct Item {
   pub token: String,
   pub value: String
}
