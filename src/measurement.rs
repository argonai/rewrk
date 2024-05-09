#![allow(unused)]
use std::{fs::File, io::Write, time::Duration};


#[derive(Default)]
pub struct Measurement {
    pub start_time :  u128,
    pub recorded_latency: Duration,
} 
impl Measurement{

    pub fn write_result(&self, file: &mut File){
        let line = format!("{},{}\n", self.start_time, self.recorded_latency.as_nanos() );
        let _ = file.write(line.as_bytes());
    }
}
