
#[derive(Debug)]
pub enum Value {
   
   KeyWord(String),
   Bool(bool),
   Integer(i32),
   EOF
}

#[derive(Debug)]
pub struct Entry {

   pub key: String,
   pub value: Value
}

#[derive(Debug)]
pub struct Setting {

   pub weather: bool,
   pub time: bool,
   pub ambiant: bool,
   pub source: String,
   pub sources: Vec<String>,
   pub resize_mode: bool,
   pub width: u32,
   pub height: u32,
}

#[derive(Debug)]
pub struct Parser<'lt> {

   pub input: &'lt str
}
