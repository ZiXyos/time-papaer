use crate::config::types::Setting; 

#[derive(Debug, Clone, Copy)]
pub enum Mood {
    
    Noon,
    AfterNoon,
    Morning,
    Night
}

#[derive(Debug, Clone, Copy)]
pub struct Paper<'lt> {

    pub setting: Setting,
    pub curr_wallpaper: &'lt str,
    pub mood: Mood
}
