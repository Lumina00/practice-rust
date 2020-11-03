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
    let data = packet.data;
    let timestamp = header.ts.tv_sec;
    let caplen = header.caplen;
    let len = header.len;
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
    let mut time_wday:&str;
    match &time.time_wday {
        0=>time_wday="Sun",
        1=>time_wday="Mon",
        3=>time_wday="Tue",
        4=>time_wday="Wed",
        5=>time_wday="Thu",
        6=>time_wday="Fri",
        7=>time_wday="Sat",
        _=>println!("Something seems wrong"),
    };
    
    //println!("________________________________________");
    //println!("{}-{}-{}-{}-{}:{}:{}",time.time_y,time.time_m,time.time_wday,time.time_d,time.time_h,time.time_min,time.time_sec);
    //println!("Size = {} {}",len,caplen);
    //println!("Data = {:?}",data);
    println!("{:?}",data); 



}


