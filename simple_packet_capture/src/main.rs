use pcap::{Device,PacketHeader,Capture};
use libc;
struct Times {
    time_y: i32,
    time_m: i32,
    time_d: i32,
    time_h: i32,
    time_mm: i32,
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

    unsafe { 
        tm = libc::localtime(&timestamp);
    }
    println!("{:?}",tm);
}

