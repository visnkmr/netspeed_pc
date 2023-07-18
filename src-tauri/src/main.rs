// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, env, process, time::SystemTime};
use chrono::{DateTime, Utc, Local};
// use ns_sse::*;
use tauri::{Manager, AppHandle, Window, Size, LogicalSize, CustomMenuItem, Submenu, GlobalWindowEvent, WindowEvent, Menu, SystemTray, SystemTrayMenu};

// // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// #[tauri::command]
// fn greet(name: &str) -> String {
//     format!("{}", name)
// }
// #[tauri::command]
// fn button1_clicked() -> String {
//     format!("start")
// }
#[tauri::command]
fn exit(app: Window) {
    println!("exit callled");
    app.hide();
  // app.get_window();
}
#[tauri::command]
fn mini(app: AppHandle) ->Result<(),()>{
  let k= app.get_window("main").unwrap();
    println!("minimise callled");
  k.minimize().unwrap();
  Ok(())
}
#[tauri::command]
fn startmove(window: Window){
    // start dragging the window when the button is clicked
    window.start_dragging().unwrap();
    // window.set_size(Size::Logical(LogicalSize { width: 100.0, height: 100.0 })).unwrap();
    // get a handle to the current window
// let window = Window::current();

// get the current window size in pixels
// let size = window.inner_size().unwrap();
// println!("{:?}", size);

// // get the current window size in logical units
// let factor = window.scale_factor().unwrap();
// // let logical = size.to_logical(factor);
// println!("{:?}", factor);
    // format!("start")
  }
//   #[tauri::command]
// fn stopmove(window: Window){
//     // start dragging the window when the button is clicked
//     window.stop_dragging().unwrap();
//     // format!("start")
//   }
const appname: &str = "ns_gui_sse";

fn main() {
  std::env::set_var("GTK_OVERLAY_SCROLLING", "0");
  human_panic::setup_panic!(human_panic::Metadata {
    version: env!("CARGO_PKG_VERSION").into(),
    name: env!("CARGO_PKG_NAME").into(),
    authors: env!("CARGO_PKG_AUTHORS").replace(":", ", ").into(),
    homepage: env!("CARGO_PKG_HOMEPAGE").into(),
    path_to_save_log_to: prefstore::prefstore_directory(&appname.to_string()).unwrap(),
});
let mut iname=String::new();
    // let date = Local::now();
    // let current_date = date.format("%Y-%m-%d").to_string();
    // setappname(var("CARGO_PKG_NAME").unwrap_or_else(|_| env!("CARGO_PKG_NAME").to_string()));
    // println!("{}",byte_unit::Byte::from_bytes(getpreference(APPNAME,&current_date,0 as u128).parse::<u128>().unwrap()).get_appropriate_unit(true));
    //set interface name from commandline to set it to track only specific network interface
    let args: Vec<String> = env::args().collect();
    match args.get(1){
        Some(g)=>{
            iname=g.to_owned();
             if(iname=="tu"){
                ns_sse::serve();
                process::exit(0);
            }
        },
        None=>{
            iname="all".to_string();
        }
    }
  thread::spawn(move ||{
    ns_sse::startserving(iname);
  });
  
    let app=tauri::Builder::default()
    .setup(|app| {
      
      // let main_window=app.get_window("main").unwrap();
      // main_window
      // .on_menu_event(|event| {
      //   match event.menu_item_id() {
      //     "reload" => {
      //       std::process::exit(0);
      //     }
      //     "close" => {
      //       // main_window.close();
      //       // event.window().close().unwrap();
      //     }
      //     "otb"=>{
      //       // otb(event.window().label(),g);
  
      //     }
      //     "Learn More" => {
      //         let url = "https://github.com/visnkmr/iomer";
      //         // shell::open(&event.shell_scope(), url.to_string(), None).unwrap();
      //       }
      //     _ => {}
      //   }
      // });
      // println!("{:?}",app);
      
      // let handle = app.handle();
    // std::thread::spawn(move || {
    //   let window = tauri::WindowBuilder::new(
    //     &handle,
    //     "label",
    //     tauri::WindowUrl::App("index.html".into())
    //   ).build().unwrap();
    // });
    // let window = tauri::WindowBuilder::new(app, "label", tauri::WindowUrl::App("index.html".into()))
    // .build()
    // .unwrap();
    let app_handle = app.handle();
    let tray_id = "my-tray";
    SystemTray::new()
      .with_id(tray_id)
      .with_menu(
        SystemTrayMenu::new()
          .add_item(CustomMenuItem::new("quit", "Quit"))
          .add_item(CustomMenuItem::new("open", "Open"))
      )
      .on_event({
        
        move |event| {
        match event{
            tauri::SystemTrayEvent::MenuItemClick { tray_id, id,.. } => {
              
              if(id=="quit"){
                

                std::process::exit(0);
              }
              else{
                // println!("{:?}",gk);
                let absolute_date=getuniquewindowlabel();
                // app.get_window("main").unwrap().show();
                showwindow(&app_handle).unwrap();

                
                // tauri::Builder::new()
                // // .manage(gk)
                // .invoke_handler(
                //   tauri::generate_handler![
                //     list_files,
                //     ]
                //   )
                // .run(tauri::generate_context!())
                // .expect("error while running tauri application");
              }

            },
            _ =>{
              //on right click on tray icon on windows this is triggered.
            },
        }
        // let tray_handle = app_handle.tray_handle_by_id(tray_id).unwrap();
        
      }
    })
      .build(app)?;
    
      // get an instance of AppHandle
      // let app_handle = app.handle().get_window("main").unwrap();
      // let g=app.state::<FileSizeFinder>();
    //   // spawn a thread to list the files in the current directory on startup
    // //   std::thread::spawn(move || {
    // //     list_files(".".to_string(), app_handle.get_window("main").unwrap());
    // //   });
    //   // set the window flags to remove WS_MAXIMIZEBOX
    //   app_handle.set_window_flags(|flags| flags & !WS_MAXIMIZEBOX)?;
    
      Ok(())
    })
   
    .on_window_event(on_window_event)

  
        .invoke_handler(tauri::generate_handler!
            [
            // greet,
            // button1_clicked,
            // button2_clicked,
            startmove,
            exit,
            mini
            ]
        )
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

      
}
fn on_window_event(event: GlobalWindowEvent) {
  if let WindowEvent::CloseRequested {
      #[cfg(not(target_os = "linux"))]
      api,
      ..
  } = event.event()
  {

      // #[cfg(target_os = "macos")]
      // {
      //     app.hide().unwrap();
      //     api.prevent_close();
      // }
  }
}

// #[tauri::command]
// fn button1_clicked() {
//   eprintln!("Button 1 clicked!");
// }

// #[tauri::command]
// fn button2_clicked() {
//   eprintln!("Button 2 clicked!");
// }

pub fn showwindow(app_handle:&AppHandle)->Result<(), tauri::Error>{
  app_handle.get_window("main").unwrap().show()
  // let INIT_SCRIPT= [r#"
  //             console.log("poiu");
  //              let kpg="#,pathtt,r#"
  //                 "#].concat();
                // tauri::WindowBuilder::new(
                //   app_handle,
                //   label,
                //   tauri::WindowUrl::App("index.html".into())
                // )
                
                // // .initialization_script(&INIT_SCRIPT)
                // .title(title).build().unwrap()
}

#[tauri::command]
fn getuniquewindowlabel()->String{
  let now = SystemTime::now();

                let now_date = DateTime::<Utc>::from(now).with_timezone(&Local);
                let absolute_date = now_date.format("%d%m%H%M%S").to_string();
                // println!("{absolute_date}");
                absolute_date
}
