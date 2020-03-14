extern crate reqwest;

use std::{thread, time};
use std::fs::File;
use std::io::prelude::*;
use std::env;


fn main() {

    let activate_connection_check: bool;
    let mut check_url = String::from("");
    
    let key = "WATCHDOG_URL";

    match env::var_os(key) {
        Some(val) => {
            
            match val.into_string(){
                Ok(value_str) =>{
                    check_url = value_str;
                    activate_connection_check = true;
                    println!("Using URL: {} ", check_url);
                },
                Err(_) => {
                    activate_connection_check = false;
                    println!("Could not transform Environment variable string.");
                }
            }
        },
        None => {
            activate_connection_check = false;
            println!("Environment variable for URL not set. Disabling keep-alive check.");
        }
    }


    let start_delay = time::Duration::from_secs(30);
    let trigger_delay = time::Duration::from_secs(5);
    println!("Started");
    thread::sleep(start_delay);
    println!("Executing request");

    let mut connection_alive =false;
    let mut error_count =0;
    let mut trigger_reboot = false;

    while trigger_reboot == false{
       
       
        if activate_connection_check == true {
           connection_alive = check_connection(&check_url);
           println!("Received Result: {} " , connection_alive);
           if connection_alive == true {
               error_count = 0;
           } else {
               error_count = error_count + 1;
           }

           if error_count >= 4 {
               trigger_reboot = true;
           }
       }
       
        if connection_alive == true || activate_connection_check == false {
            println!("reset watchdog");

            let mut file = File::create("/dev/watchdog")
            .expect("Error opening file");


            file.write_all(b"watch, dog!")
            .expect("Error writing to file");
        }

        thread::sleep(trigger_delay);
    }

}


fn check_connection(myurl: &str) -> bool{

    let mut result=true;
    match reqwest::blocking::get(myurl){
        Ok(response) =>{
            if response.status() == 200{
                println!("Success");
                result=true;
            }
        },
        Err(_) => {
            println!("Error");
            result=false;
          }
    }
  return result;
}