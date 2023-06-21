use std::{error::Error, sync::{Arc, Mutex}, clone, future::Future, pin::Pin};
use crate::config::types::Setting;
use super::types::{Paper, Mood};
use chrono::prelude::*;
use tokio_cron_scheduler::{JobScheduler, Job};

use wallpaper;

impl<'lt> Paper<'lt> {
    
    pub fn new(setting: Setting) -> Paper<'lt> {

        let curr = wallpaper::get().unwrap_or(String::from(""));
        let curr_ref: &'lt str = Box::leak(curr.into_boxed_str());
        Paper { setting, curr_wallpaper: curr_ref, mood: Mood::Morning}
    }

    pub fn set_wallpaper(fpath: String) -> Result<String, Box<dyn Error>> {

       match wallpaper::set_from_path(&fpath) {
        
            Ok(_) => match wallpaper::get() {
                
                Ok(fpath) => Ok(fpath),
                Err(e) => Err(e)
            }
            Err(e) => return Err(e) 
       }
    }

    pub fn get_mood(&mut self) -> Result<Mood, Box<dyn Error>> {

        let current_hour = Utc::now().hour();
        match current_hour {
            5..=11 => Ok(Mood::Morning),
            12..=16 => Ok(Mood::Noon),
            17..=23 => Ok(Mood::AfterNoon),
            0..=4 => Ok(Mood::Morning),
            _ => panic!("wtf is da numbs")
        }
    }

    pub fn set_mood(&mut self) {

        let current_mood = self.get_mood().unwrap_or(Mood::Morning);
        self.mood = current_mood;
    }

    pub async fn hourly_change(&mut self, _fpath: String) {

        let self_arc = Arc::new(Mutex::new(self));
        let schedule = JobScheduler::new().await.unwrap();
        let arc_ref = Arc::clone(&self_arc);

        let  job = Job::new("1/10 * * * * *", move |_, _| {

            let current_hour = Utc::now().hour();
           // arc_ref.lock().unwrap();
        }).unwrap();

        schedule.add(job).await.unwrap();
        loop {
            schedule.start().await;
        }
    }
}
