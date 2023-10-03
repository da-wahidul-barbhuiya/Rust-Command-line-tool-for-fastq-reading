use core::panic;
use std::fmt::write;
use std::process::Output;
use std::str::FromStr;
use std::sync::Arc;
use std::{error::Error, io::BufRead};
use std::fs::File;
use std::io::BufReader;
use chrono::{prelude::*,FixedOffset,DateTime,Utc};
use regex::{Regex, Captures};
use std::io::{self, Write};
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
pub fn time_cap(input:&str) -> Option<regex::Captures<'_>>{
    let date_time_re:Regex=Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap();
    let date_time_demo=date_time_re.captures(&input).unwrap();
    Some(date_time_demo)
    // lazy_static!{
    //     static ref Re:Regex=Regex::new(r"start_time=(?P<time>\S+)\s*").unwrap()
    // }
    // Re.captures(input).and_then(|cap|{
    //     cap.name("time").map()
    //     println!("Hello world")
    // })
}
//pub time_cursed()
pub fn extract<'a>(time_hr:i64,contents:BufReader<File>) -> Vec<(String,String,String,String,String)>{
    //initial vector for storing 4 different lines from a single read and store those as a tuple 
    let mut store_all:Vec<(String,String,String,String,String)>=Vec::new();
    //making a temp tuple for storing all 4 lines as an element from each iterator 
    let mut current_tuple:(String,String,String,String,String)=
    (String::new(),String::new(),String::new(),String::new(),String::new());
    //This empty vector will store after applying time_hr argument
    //let mut final_vec:Vec<(DateTime<FixedOffset>,String, String, String, String)>=Vec::new();
    let mut final_vec:Vec<(String, String, String, String,String)>=Vec::new();
    //reading file for different lines using differnt condition
    for line_result in contents.lines(){
        let line =line_result.unwrap();
        if line.starts_with("@") {
            //let date_time=time_cap(&line) else{return vec![(chrono::DateTime<FixedOffset>,String::new(),String::new(),String::new(),String::new())];};
            //current_tuple.0=date_time;
            let date_time=time_cap(&line);
            //let date_time_str=date_time
            let date_time_str: String = match date_time {
                Some(captures) => captures["time"].to_string(),
                None => panic!("Default value for None case")//String::from("Default Value for None Case"),
            };
            //println!("Time from each sequence:{:?}",date_time_str);
            current_tuple.1 = line.to_string();
            current_tuple.0=date_time_str;
        } else if line.starts_with("A") || line.starts_with("T") || line.starts_with("G") || line.starts_with("C") {
            current_tuple.2 = line.to_string();
        } else if line.starts_with("+") {
            current_tuple.3 = line.to_string();
        } else {
            current_tuple.4 = line.to_string();

            store_all.push((current_tuple.0,current_tuple.1,current_tuple.2,current_tuple.3,current_tuple.4));
            current_tuple = ( String::new(), String::new(), String::new(), String::new(),String::new());
         }
         //println!("Current tuple is :{:?}",current_tuple);
    }
    //sorting using timestamp for getting start time in first place
    store_all.sort_by(|a,b|a.0.cmp(&b.0));
    let mut date_time1=chrono::Utc::now();

    let first_date_time_str=&store_all[0].0;
    let first_date_time=DateTime::<FixedOffset>::from_str(&first_date_time_str).unwrap();
    let mut hour_later=first_date_time+chrono::Duration::hours(time_hr);
    let mut out_put_file=File::create("output.fastq").expect("Failed to create a file in fastq format");

    for (timestamp,element2,element3,element4,element5) in store_all.iter(){
        let current_time=DateTime::<FixedOffset>::from_str(&timestamp).unwrap();
        if current_time<=hour_later{
            let header = format!("@{}\n", timestamp);
            //println!("Current time : {}",header);
            let sequence = format!("{}\n", element2);
            let quality = format!("{}\n", element5);

            // Write the FASTQ record to the file
            write_fastq_record(&mut out_put_file, &header, &sequence, &quality)
                .expect("Failed to write FASTQ record");
        }
    }
    println!("Extraction is done for the time stamp of {} hr",time_hr);
    store_all
   
}
//fn vector_to_writing_file(vec)

fn write_fastq_record<W>(
    writer:&mut W,
    header:&str,
    sequence:&str,
    quality:&str,
) ->io::Result<()>
where
W:Write,
{
    writer.write_all(header.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(sequence.as_bytes())?;
    writer.write_all(b"\n")?;
    writer.write_all(b"+\n")?;
    writer.write_all(quality.as_bytes())?;
    writer.write_all(b"\n")?;
    Ok(())
}
fn in_between_two_time(){

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



