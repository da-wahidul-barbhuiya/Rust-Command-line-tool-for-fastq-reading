use std::env;
use std::process;
use test_buffer::Config;

fn main() {
    let args:Vec<String>=env::args().collect();

    let config=Config::build(&args).unwrap_or_else(|err|{
        println!("Problem parsing arguments:{err}");
        process::exit(1);
    });
    println!("Time Stamp:{} hr",config.time_hr);
    println!("File name:{:?}",config.file_name);
    if let Err(e)=test_buffer::run(config){
        println!("Application error:{e}");
        process::exit(1);
    }

}

