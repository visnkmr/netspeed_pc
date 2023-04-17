// #![windows_subsystem = "windows"]
use std::{process::{exit},
    time::{Duration, Instant},
        thread, collections::HashMap,env::{self, var}, net::{TcpListener, TcpStream}, io::{BufRead, Write}};
// use abserde::Location;
// use byte_unit::Byte;
use chrono::Local;
// mod abserdeapi;
// use fltk::{app::{App, self}, window::{Window, OverlayWindow, self, SingleWindow, DoubleWindow}, prelude::*, enums::{Color, self}, text::{TextDisplay, TextBuffer}, frame, menu};
use sysinfo::{SystemExt, NetworkExt, System};
// use abserde::*;
// use abserdeapi::*;
use prefstore::*;
const APPNAME:&str="ns_sse_aio";
pub fn serve(){
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d").to_string();
    println!("{}",byte_unit::Byte::from_bytes(
        getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap()
    ).get_appropriate_unit(true));
}
pub fn startserving(iname:String) {
    
    //to store interface name
    
        
    let mut dtpr:Vec<u64>=vec![0,0,0]; 
    let date = Local::now();
    let mut current_date = date.format("%Y-%m-%d").to_string();
    let mut last_saved_date:String =String::new();
    // println!("fromhere------------>4");
    // dtpr[0] = getpreference(APPNAME,&current_date,0 as u64).parse::<u64>().unwrap();//stores total upload and download bytes count of current session and total data usage since the start of the ns_daemon in a day
    dtpr[0] = 0 as u64;//stores total upload and download bytes count of current session and total data usage since the start of the ns_daemon in a day
    let ina=iname.clone();
    thread::spawn(move || loop {
        // println!("fromhere------------>1");
        updateusage(true/*,&mut val,&mut ptx,&mut prx*/,ina.to_owned(),&mut dtpr,&mut last_saved_date);
        thread::sleep(Duration::from_secs(60));
    });
    let mut sys = System::new();
    let mut tt:u128=getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap();
    let mut perminute=0;
    //customize port and address here.
    match TcpListener::bind("127.0.0.1:7798") {
        Ok(listener) =>{
            for stream in listener.incoming(){
                let stream = stream.unwrap();
                    handle_con(stream,iname.clone(),&mut sys,&mut tt,&mut perminute);
            }
        },
        Err(e) =>{
            println!("Internet issue.\n Error:{}",e)
        }
    }
    
}
fn handle_con(mut stream:TcpStream,iname:String,sys:&mut System,tt:&mut u128,perminute:&mut i32){
    let buf_reader = std::io::BufReader::new(&mut stream);
    let request_line = match buf_reader.lines().next() {
        None => "".to_string(),
        Some(secondline) => {
            match secondline {
                Ok(reqline)  => reqline,
                Err(e) => "".to_string(),
            }
        },
    };
    
    if request_line == "GET /stream HTTP/1.1".to_string() {
       {
                
                // spawn a thread to send SSE events
                // std::thread::spawn(move || 
                    {
                    println!("inthread");

                    
                stream.write_all(b"HTTP/1.1 200 OK\r\n");
                // set the content type header to text/event-stream
                stream.write_all(b"Access-Control-Allow-Origin: *\r\n");
                stream.write_all(b"Access-Control-Allow-Credentials: true\r\n");
                stream.write_all(b"Access-Control-Allow-Methods: *\r\n");
                stream.write_all(b"Access-Control-Allow-Headers: *\r\n");
                stream.write_all(b"Content-Type: text/event-stream\r\n");
                // mark the end of headers
                stream.write_all(b"\r\n");
                    // handle errors
                    // let mut closecon=false;
                    // initialize a counter and a start time
                    let mut counter = 0;
                    let start = Instant::now();
                    
                    // loop forever
                    loop 
                    // while(rx.recv().unwrap())
                    {   
                        // println!("{}",closecon);
                        
                        // wait for one second between each event
                        
                        // increment the counter
                        counter += 1;
                        // calculate the elapsed time since start
                        let elapsed = start.elapsed().as_secs();
                        // println!("{}",marks(&iname,sys,tt,perminute));
                        // format the event data as a string
                        let data = format!(
                            // "id: {}\nevent: message\ndata: {{\"counter\": {}, \"elapsed\": {}}}\n\n",
                            "id: {}\nevent: message\ndata: {}\n\n",
                            counter,forsse(&iname,sys,tt,perminute)
                        );
                        println!("{}",data);
                        // write the data to the stream
                        match stream.write_all(data.as_bytes()) {
                            Ok(_) => {
                                // flush the stream
                                stream.flush();
                                std::thread::sleep(Duration::from_secs(1));
                                
                                
                            }
                            Err(e) => {
                                // handle write errors (e.g. client disconnected)
                                println!("Write error: {}", e);
                                break ;
                            }
                        }
                        // if(closecon){
                        //     println!("here");   
                        //     break

                        //         // std::process::exit(0)
                            
                        // }
                        
                    }
                }
            // );
            
        }
        
  
        }
        else{
            let (status_line, filecontent,contentheader) =
            if request_line == "GET / HTTP/1.1".to_string() {
                ("HTTP/1.1 200 OK", marks(&iname,sys,tt,perminute),String::from("Content-Type: application/json"))
            }
            else{
                ("HTTP/1.1 200 OK", sincelastread(),String::from("Content-Type: application/json"))
            };
            let response =
            format!("{status_line}\r\n{contentheader}\r\n\r\n{filecontent}");
            match stream.write(response.as_bytes()) {
                Ok(file) => {
                },
                Err(error) =>{
                    return ;
                },
            };
            
            match stream.flush() {
                Ok(file) => {
                },
                Err(error) =>{
                    return ;
                },
            };
        }
    }
    //returns total upload and download bytes count of current session and total data usage since the start of the ns_daemon in a day
    pub fn marks(iname:&String,sys:&mut System,tt:&mut u128,perminute:&mut i32)->String{
                    sys.refresh_networks_list();
                    
                    let mut total_rx: u64 = 0;
                    let mut total_tx: u64 = 0;
                    let networks = sys.networks();
                    for (name, network) in networks {
                            let mut nametostat=iname.as_str();
                            if(nametostat=="all"){
                            total_rx += network.total_received();
                            total_tx += network.total_transmitted();
                            }
                            else if(*name == *iname){
                                total_rx += network.total_received();
                                total_tx += network.total_transmitted();
                                break;
                            }
                    }
                    let date = Local::now();
                            let current_date = date.format("%Y-%m-%d").to_string();
                            // println!("fromhere------------>3");
                            if(*perminute>60){
                                *tt=getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap();
                                *perminute=0;
                            }
                            *perminute+=1;
                return serde_json::to_string_pretty(&vec![total_tx,total_rx,*tt as u64]).unwrap();
        }
        pub fn forsse(iname:&String,sys:&mut System,tt:&mut u128,perminute:&mut i32)->serde_json::Value{
                    sys.refresh_networks_list();
                    
                    let mut total_rx: u64 = 0;
                    let mut total_tx: u64 = 0;
                    let networks = sys.networks();
                    for (name, network) in networks {
                            let mut nametostat=iname.as_str();
                            if(nametostat=="all"){
                            total_rx += network.total_received();
                            total_tx += network.total_transmitted();
                            }
                            else if(*name == *iname){
                                total_rx += network.total_received();
                                total_tx += network.total_transmitted();
                                break;
                            }
                    }
                    let date = Local::now();
                            let current_date = date.format("%Y-%m-%d").to_string();
                            // println!("fromhere------------>3");
                            if(*perminute>60){
                                *tt=getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap();
                                *perminute=0;
                            }
                            *perminute+=1;
                // return total_tx.to_string();
                return serde_json::to_value(&vec![total_tx,total_rx,*tt as u64]).unwrap();

        }
//returns todays total while ns_daemon running
pub fn sincelastread()->String{
    let date = Local::now();
    let current_date = date.format("%Y-%m-%d").to_string();
    let tt=getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap();
    return serde_json::to_string_pretty(&vec![tt as u64]).unwrap();
}
// saves bytes used every minute to file while ns_daemon running
fn updateusage(whethertosave:bool/*,val:&mut u128,ptx:&mut u64,prx:&mut u64*/,iname:String,dtpr:&mut Vec<u64>,lastsaveddate:&mut String){//->String{
            
            let date = Local::now();
            let current_date = date.format("%Y-%m-%d").to_string();
            if *lastsaveddate!=current_date{
                dtpr[0]=getpreference(APPNAME,&current_date,0 as u64).parse::<u64>().unwrap()
            }
            let mut sys = System::new();
            sys.refresh_networks_list();
            let mut total_rx: u64 = 0;
            let mut couldfind=false;
            let mut total_tx: u64 = 0;
            let networks = sys.networks();
            let mut k=0;
            for (name, network) in networks {
                    if(iname=="all"){
                    total_rx += network.total_received();
                    total_tx += network.total_transmitted();
                    couldfind=true;
                    }
                    else if(*name == *iname){
                        total_rx += network.total_received();
                        total_tx += network.total_transmitted();
                        couldfind=true;
                        break;
                    }
            }
            let mut turx=total_rx.saturating_sub(dtpr[2]);
            let mut tutx=total_tx.saturating_sub(dtpr[1]);
            if dtpr[1]!=0 ||dtpr[2]!=0 {
                dtpr[0]+=turx+tutx;
                // let mut dm:HashMap<String,u128>=HashMap::new();
            
            {
                if whethertosave{
                savepreference(APPNAME,&current_date, dtpr[0] as u128)
            }
            }
            
            }
            dtpr[1]=total_tx;
            dtpr[2]=total_rx;
            *lastsaveddate = current_date;
            // let tt=total_rx+total_tx;
            // let byte_rx = byte_unit::Byte::from_bytes(turx as u128);
            // let byte_tx = byte_unit::Byte::from_bytes(tutx as u128);
            // let byte_t = byte_unit::Byte::from_bytes(tt as u128);
            // let adjusted_rx = byte_rx.get_appropriate_unit(true);
            // let adjusted_tx = byte_tx.get_appropriate_unit(true);
            // let adjusted_st = byte_unit::Byte::from_bytes(dtpr[0] as u128).get_appropriate_unit(true);
            // if couldfind{
            //     format!("{}↓ {}↑ {}",adjusted_rx,adjusted_tx,adjusted_st )
            // }
            // else{
            //     format!("No such network")
            // }
}
// returns total data used since session started / present logged in session usage
// fn updateupdowndatausg(ptx:&mut u64,prx:&mut u64)->u128{
//     let mut sys = System::new();
//             sys.refresh_networks_list();
//             let mut total_rx: u64 = 0;
//             let mut total_tx: u64 = 0;
//             let networks = sys.networks();
//             let mut k=0;
//             for (name, network) in networks {
//                     {
//                     total_rx += network.total_received();
//                     total_tx += network.total_transmitted();
//                     k=1;
//                     }
//             }
//             let mut turx=total_rx.saturating_sub(*prx);
//             let mut tutx=total_tx.saturating_sub(*ptx);
//             *ptx=total_tx;
//             *prx=total_rx;
//             let tt=total_rx+total_tx;
//             tt as u128
// }
