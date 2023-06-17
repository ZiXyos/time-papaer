#[derive(Debug)]
pub struct Config {
    pub mode: Mode,
    pub is_active: bool,
}

#[derive(Debug, Default)]
pub struct Lexer<'lt> {
   pub input: &'lt str,
   pub pos: usize
}

pub enum Token {
    
   Equal,
   KeyWord(String),
   EOF
}

#[derive(Debug)]
pub enum Mode {

   Hourly,
   Min,
   Daily,
   Monthly,
   Ambiant
}

