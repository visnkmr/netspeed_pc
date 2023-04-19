const { invoke } = window.__TAURI__.tauri;
// import { appWindow } from '@tauri-apps/api/tauri'

// let greetInputEl;
// let greetMsgEl: Element;

// async function greet(whatever:String) {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("greet", { name: whatever });
// }
// async function button1_clicked() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   greetMsgEl.textContent = await invoke("button1_clicked");
// }
// async function button2_clicked() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   await invoke("button2_clicked");
// }
async function exit() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("exit");
}
async function minimise() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("mini");
}
async function movewindow() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  await invoke("startmove");
}
// async function stopmovewindow() {
//   // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
//   await invoke("startmove");
// }
// hidescrollbaredge()
function hidescrollbaredge(){
  document.querySelector('body').style.overflow='scroll';
  var style=document.createElement('style');
  style.type='text/css';
  style.innerHTML='::-webkit-scrollbar{display:none}';
  document.getElementsByTagName('body')[0].appendChild(style);
}

function startstopmovewindow(e: MouseEvent) {
    // get the window element
    let window = document.getElementById("window");
    // get the event coordinates
    let x = e.clientX;
    let y = e.clientY;
    // get the window dimensions
    let w = window.offsetWidth;
    let h = window.offsetHeight;
    // define a threshold for resizing
    let t = 1;
    // check if the event is within the threshold of the window edges
    let resize = (x < t || x > w - t || y < t || y > h - t);
    // if not resizing and not clicking a button, then move the window
    if (e.buttons === 1 && e.target?.tagName !== 'BUTTON' && !resize) {
      movewindow();
    }
  }
window.addEventListener('mousedown', (e) => startstopmovewindow(e));
window.addEventListener('mouseup', (e) => startstopmovewindow(e));

window.addEventListener("DOMContentLoaded", () => {
  
  // greetInputEl = document.querySelector("#greet-input");
  // greetMsgEl = document.querySelector("#greet-msg") as HTMLParagraphElement;
  // document
  //   .querySelector("#greet-button")
  //   .addEventListener("click", () => greet(greetInputEl.value));
  
  // document
  //   .querySelector("#other")
  //   ?.addEventListener("mousedown", () => movewindow());
  // document
  //   .querySelector("#other")
  //   ?.addEventListener("mouseup", () => stopmovewindow());
  //   document
  //   .querySelector("#start")
  //   ?.addEventListener("click", () => button2_clicked());
    // .addEventListener("click", () => greet("start"));
    document
    .querySelector("#stop")
    // .addEventListener("click", () => greet("stop"));
    // .addEventListener("click", () => button2_clicked());
    // .addEventListener("mousedown", () => appWindow.startDragging());
    ?.addEventListener("click", () => exit());
    document
    .querySelector("#mini")
    // .addEventListener("click", () => greet("stop"));
    // .addEventListener("click", () => button2_clicked());
    // .addEventListener("mousedown", () => appWindow.startDragging());
    ?.addEventListener("click", () => minimise());
});
let last_upload: number=0, last_download:number=0, upload_speed:number, down_speed:number

ssestart()

function ssestart(){
  const source = new EventSource("http://127.0.0.1:7798/stream");

// listen for messages
  source.onmessage = function(event) {
    console.log(event)
    // parse the JSON data
    const data = JSON.parse(event.data);
    let upload=data[0]
    let download=data[1]
    let todaytotal=data[2]

    if (last_upload > 0){
       if( upload < last_upload)
                upload_speed = 0
            else
                upload_speed = upload - last_upload
    }
           

        if (last_download > 0){
           if (download < last_download)
                down_speed = 0
            else
                down_speed = download - last_download
        }
           

        last_upload = upload
        last_download = download
    // display the data in HTML
    document.getElementById("showspeed")!.innerHTML = size(down_speed,false)+"ps↓ "+ size(upload_speed,false)+"ps↑ "+ size(todaytotal,true);
    // document.getElementById("showspeed").innerHTML =down_speed+"ps↓ "+ upload_speed+"ps↑ "+ todaytotal;
  };
}

// function size(speed:number,isbytes:boolean){
//   if(speed===null)
//   speed=0
//   return speed
// }
// Variables for use in the size() function.
const KB = 1024;
const MB = KB ** 2; // 1,048,576
const GB = KB ** 3; // 1,073,741,824
const TB = KB ** 4; // 1,099,511,627,776

function size(B: number, isbytes: boolean): string {
  B = isbytes ? B : B * 8;
  const u = isbytes ? "B" : "b";
  if (B < KB) {
    return `${Math.trunc(B)}${u}`;
  } else if (KB <= B && B < MB) {
    return `${Math.trunc(B / KB)} K${u}`;
  } else if (MB <= B && B < GB) {
    return `${Math.trunc(B / MB)} M${u}`;
  } else if (GB <= B && B < TB) {
    return `${Math.trunc(B / GB)} G${u}`;
  } else if (TB <= B) {
    return `${Math.trunc(B / TB)} T${u}`;
  }
  return ""
}