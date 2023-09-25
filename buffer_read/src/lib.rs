use core::panic;
use std::sync::Arc;
use std::{error::Error, io::BufRead};
use std::fs::File;
use std::io::BufReader;
use chrono::{prelude::*,FixedOffset,DateTime,Utc};
use regex::{Regex, Captures};
// creating struct for storing two arguments
pub struct  Config{
    pub time_hr:i64,
    pub file_name:File,
}

//implementing build function with struct with passing generic type as both args are of different types(i64 and String)
impl Config{
    pub fn build<a>(args:&[a]) ->Result<Config,&'static str>
    where
    a:AsRef<str>{
        if args.len()<3{
            return  Err("Not enough arguments")
        }
        //converting time_hr to String type
        let time_hr=args[1].as_ref().to_owned();
        //converting time_hr from String to i64 type
        let time_hr_i64: i64 = time_hr.trim().parse().map_err(|_| "Failed to parse time_hr")?;
    
        //let time_hr:i64=time_hr.trim().parse();
        //let file_name=args[2].clone();
        // converting file_name to String type
        let file_name=args[2].as_ref();
        let file=File::open(file_name).map_err(|_| "Failed to open file")?;
        Ok(Config { time_hr: time_hr_i64,file_name:file })
        

    }
}

pub fn run(config:Config) ->Result<(),Box<dyn Error>>{
    //let f=File::open(config.file_name)?;
    let  contents=BufReader::new(config.file_name);
    //reading file contents and passing as second argument
    //calling extract function where as time_hr is passing as field
    extract(config.time_hr, contents);
    Ok(())

}
//pub time_cursed()
pub fn extract<'a>(time_hr:i64,contents:BufReader<File>) -> Vec<(DateTime<FixedOffset>,String,String,String,String)>{
    //initial vector for storing 4 different lines from a single read and store those as a tuple 
    let mut store_all:Vec<(DateTime<FixedOffset>,String,String,String,String)>=Vec::new();
    //making a temp vector for storing all 4 lines in each iterator as an tuple element
    let mut current_tuple:(Option<regex::Captures>,String,String,String,String)=
    (None,String::new(),String::new(),String::new(),String::new());
    //This empty vector will store after applying time_hr argument
    //let mut final_vec:Vec<(DateTime<FixedOffset>,String, String, String, String)>=Vec::new();
    let mut final_vec:Vec<(DateTime<FixedOffset>,String, String, String, String)>=Vec::new();
    //reading file for different lines using differnt condition
    for line_result in contents.lines(){
        let line =line_result.unwrap();
        if line.starts_with("@") {
            let date_time_re:Regex=Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap(); 
            let line_str=line.to_string();
            let parsi="start_time=(?P<time>";
            //let Some(date_time)=parsi.captures(date_time_re) else {return };
            if let date_time_new=date_time_re.captures(&line_str){
                let Some(date_time)=date_time_new.(date_time_re).unwrap();
                current_tuple.0=Some(date_time);
            } //else{ panic!("No match!"); };
            
            current_tuple.1 = line.to_string();
        } else if line.starts_with("A") || line.starts_with("T") || line.starts_with("G") || line.starts_with("C") {
            current_tuple.2 = line.to_string();
        } else if line.starts_with("+") {
            current_tuple.3 = line.to_string();
        } else {
            current_tuple.4 = line.to_string();
            if let Some(captures)=current_tuple.0.take(){
                let time=captures["time"].to_string();
                let date_time=DateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S.%f%z").expect("Failed to parse date time");
                store_all.push((date_time, current_tuple.1.clone(), current_tuple.2.clone(), current_tuple.3.clone(), current_tuple.4.clone()));
            }
            current_tuple = (None, String::new(), String::new(), String::new(), String::new());
         }
         println!("Current tuple is :{:?}",current_tuple);
    
    }
    //sorting using timestamp for getting start time in first place
    store_all.sort_by(|a,b|a.0.cmp(&b.0));
    let mut date_time1=chrono::Utc::now();
    //let mut datetime1 = FixedOffset::east(5 * 3600 + 30 * 60); // +5 hours and 30 minutes
    println!("store_all:{:?}",store_all);
    if let Some((first_datetime1, _, _, _, _)) = store_all.first() {
        // 'datetime' now contains the first element of the first tuple
        let p = *first_datetime1;
        date_time1=p.into();
        // date_time1=p_con;
    

     } else {
        // Handle the case where the vector is empty
        panic!("Vector is empty");
    }
    // extending time interval from start time 
    let date_time1_change:DateTime<FixedOffset>=date_time1.into();
    let one_hour_later=date_time1_change+chrono::Duration::hours(time_hr);
    for (timestamp,element2,element3,element4,element5) in store_all.iter()
    {
        if timestamp <= &one_hour_later{
            //println!("Timestamp:{:?},Element2:{:?},Element3:{:?},Element4:{:?},Element5:{:?}",time_hr,element2,element3,element4,element5);
            //storing final output to the vector again
            final_vec.push((*timestamp,element2.clone(),element3.clone(),element4.clone(),element5.clone()));
            
        }
    }
    println!("final vector:{:?}",final_vec);    
    final_vec
}
// #[cfg(test)]
// mod test{
//     use super::*;

//     #[test]
//     fn one_result(){
//         let time_hr="2023";
//         let contents="\
//     Rust:
//     safe,fast,productive.
//     pick three.";
//     assert_eq!(vec!["header,sequence,productive."],extract(time_hr,contents));

//     }
//}
//how to make this test successfull


