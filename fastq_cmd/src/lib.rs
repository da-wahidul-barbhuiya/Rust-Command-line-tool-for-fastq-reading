use std::error::Error;
use std::fs;
use chrono::{prelude::*,FixedOffset};
// creating struct for storing two arguments
pub struct  Config{
    pub time_hr:i64,
    pub file_name:String,
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
        let file_name=args[2].as_ref().to_owned();
        Ok(Config { time_hr: time_hr_i64,file_name })
        

    }
}

pub fn run(config:Config) ->Result<(),Box<dyn Error>>{
    let contents=fs::read_to_string(config.file_name)?;
    //reading file contents and passing as second argument
    //calling extract function where as time_hr is passing as field
    extract(config.time_hr, &contents);
    Ok(())

}
pub fn extract<'a>(time_hr:i64,contents:&'a str) -> Vec<(DateTime<FixedOffset>,String,String,String,String)>{
    //initial vector for storing 4 different lines from a single read and store those as a tuple 
    let mut store_all:Vec<(DateTime<FixedOffset>,String,String,String,String)>=Vec::new();
    //making a temp vector for storing all 4 lines in each iterator as an tuple element
    let mut current_tuple:(DateTime<FixedOffset>,String,String,String,String)=
    (Default::default(),Default::default(), Default::default(), Default::default(), Default::default());
    //This empty vector will store after applying time_hr argument
    let mut final_vec:Vec<(DateTime<FixedOffset>,String, String, String, String)>=Vec::new();
    //reading file for different lines using differnt condition
    for line in contents.lines(){
        if line.starts_with("@") {
            let start_index = line.to_string().find("2023").unwrap_or(0);
            let end_index = line.to_string().find(" flow_cell_id").unwrap_or(line.to_string().len());
            let result = &line.to_string()[start_index..end_index];
            let date_time = DateTime::parse_from_rfc3339(result).unwrap();
            // reads.push(date_time);
            current_tuple.0=date_time;
            current_tuple.1 = line.to_string();
        } else if line.starts_with("A") || line.starts_with("T") || line.starts_with("G") || line.starts_with("C") {
            current_tuple.2 = line.to_string();
        } else if line.starts_with("+") {
            current_tuple.3 = line.to_string();
        } else {
            current_tuple.4 = line.to_string();
            store_all.push(current_tuple);
            current_tuple = (Default::default(),Default::default(), Default::default(), Default::default(), Default::default());
        }
    }
    //sorting using timestamp for getting start time in first place
    store_all.sort_by(|a,b|a.0.cmp(&b.0));
    //taking first timestamp as reference 
    let base_timestamp = DateTime::parse_from_rfc3339("2023-06-01T12:47:06.339862+05:30")// Please change this it is for just an example
        .unwrap()
        .with_timezone(&Utc);
    // extending time interval from start time 
    let one_hour_later=base_timestamp+chrono::Duration::hours(time_hr);
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


