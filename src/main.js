"use strict";
const { invoke } = window.__TAURI__.tauri;
async function exit() {
    await invoke("exit");
}
async function minimise() {
    await invoke("mini");
}
async function movewindow() {
    await invoke("startmove");
}
function startstopmovewindow(e) {
    if (e.buttons === 1 && e.target?.tagName !== 'BUTTON') {
        movewindow();
    }
}
window.addEventListener('mousedown', (e) => startstopmovewindow(e));
window.addEventListener('mouseup', (e) => startstopmovewindow(e));
window.addEventListener("DOMContentLoaded", () => {
    document
        .querySelector("#stop")
        ?.addEventListener("click", () => exit());
    document
        .querySelector("#mini")
        ?.addEventListener("click", () => minimise());
});
let last_upload = 0, last_download = 0, upload_speed, down_speed;
ssestart();
function ssestart() {
    const source = new EventSource("http://127.0.0.1:7798/stream");
    source.onmessage = function (event) {
        console.log(event);
        const data = JSON.parse(event.data);
        let upload = data[0];
        let download = data[1];
        let todaytotal = data[2];
        if (last_upload > 0) {
            if (upload < last_upload)
                upload_speed = 0;
            else
                upload_speed = upload - last_upload;
        }
        if (last_download > 0) {
            if (download < last_download)
                down_speed = 0;
            else
                down_speed = download - last_download;
        }
        last_upload = upload;
        last_download = download;
        document.getElementById("showspeed").innerHTML = size(down_speed, false) + "ps↓ " + size(upload_speed, false) + "ps↑ " + size(todaytotal, true);
    };
}
const KB = 1024;
const MB = KB ** 2;
const GB = KB ** 3;
const TB = KB ** 4;
function size(B, isbytes) {
    B = isbytes ? B : B * 8;
    const u = isbytes ? "B" : "b";
    if (B < KB) {
        return `${Math.trunc(B)}${u}`;
    }
    else if (KB <= B && B < MB) {
        return `${Math.trunc(B / KB)} K${u}`;
    }
    else if (MB <= B && B < GB) {
        return `${Math.trunc(B / MB)} M${u}`;
    }
    else if (GB <= B && B < TB) {
        return `${Math.trunc(B / GB)} G${u}`;
    }
    else if (TB <= B) {
        return `${Math.trunc(B / TB)} T${u}`;
    }
    return "";
}
