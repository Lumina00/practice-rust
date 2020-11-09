use pcap::{Device,Capture};

trait Summarizable {
    fn summary(&self);
    fn w_wday(&self) -> &str;
}

struct Times {
    time_y: i32,
    time_m: i32,
    time_wday: i32,
    time_d: i32,
    time_h: i32,
    time_min: i32,
    time_sec: i32,
}
impl Summarizable for Times {
    fn summary(&self) {
    format!("{}-{}-{}-{}-{}:{}:{}",self.time_y,self.time_m,self.w_wday(),self.time_d,self.time_h,self.time_min,self.time_sec);
    }
    fn w_wday(&self) -> &str {
        let mut wday:&str;
        match self.time_wday {
            0=>wday="Sun",
            1=>wday="Mon",
            3=>wday="Tue",
            4=>wday="Wed",
            5=>wday="Thu",
            6=>wday="Fri",
            7=>wday="Sat",
            _=>panic!("Something seems wrong"),
        
            };
wday
    }
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
    
    //println!("________________________________________");
    //println!("{}",time.summary);
    //println!("Size = {} {}",len,caplen);
    //println!("Data = {:?}",data);
    println!("{:?}",data); 

}


