use pcap::{Device,Capture};
struct Times {
    time_y: i32,
    time_m: i32,
    time_wday: i32,
    time_d: i32,
    time_h: i32,
    time_min: i32,
    time_sec: i32,
}
fn main() {
    let maindevice = Device::lookup().unwrap();
    let mut cap = Capture::from_device(maindevice).unwrap()
        .promisc(true)
        .snaplen(5000)
        .open().unwrap();
    let mut packet = cap.next().unwrap();
    let header = packet.header;
    let timestamp = header.ts.tv_sec;
    let mut tm:*mut libc::tm;
    let mut time:Times;
    unsafe { 
        tm = libc::localtime(&timestamp);
    
        let _time = Times{
            time_y: (*tm).tm_year + 1900,
            time_m: (*tm).tm_mon + 1,
            time_wday: (*tm).tm_wday,  //Sunday == 0 
            time_d: (*tm).tm_mday,
            time_h: (*tm).tm_hour,
            time_min: (*tm).tm_min,
            time_sec: (*tm).tm_sec,
        };
        time = _time;

    }

    println!("{}-{}-{}-{}-{}:{}:{}",time.time_y,time.time_m,time.time_wday,time.time_d,time.time_h,time.time_min,time.time_sec);
}

