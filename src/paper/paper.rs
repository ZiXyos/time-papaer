use std::error::Error;
use crate::config::types::Setting;
use super::types::Paper;

use tokio_cron_scheduler::{JobScheduler, Job};

use wallpaper;

impl Paper {
    
    pub fn new(setting: Setting) -> Paper {

        match wallpaper::get() {
            Ok(curr) => return Paper{setting, curr_wallpaper: curr},
            Err(_) => return Paper { setting, curr_wallpaper: String::new() }
        };
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

    pub async fn hourly_change(&self, _fpath: String) {

        let schedule = JobScheduler::new().await.unwrap();
        let job = Job::new("1/10 * * * * *", |_uuid, _l| {

            println!("I run every 10 sec");
        }).unwrap();

        schedule.add(job).await.unwrap();
        loop {
            schedule.start().await;
        }
    }
}
