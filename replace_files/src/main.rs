use std::{fs,io};
use std::path::PathBuf;

fn replace(a:PathBuf,b:PathBuf) {
    let mut temp = b
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    temp.push_str(".tmp");
    let temp = PathBuf::from(temp);
    fs::copy(&a,&temp)
        .expect("Permission Denied");
    fs::remove_file(&a)
        .expect("Permission Denied");
    fs::copy(&b,&a)
        .expect("Permission Denied");
    fs::rename(&temp,&a)
        .expect("Permission Denied");
    
}
fn read() -> PathBuf{
    let mut a = String::new();
    io::stdin().read_line(&mut a)
        .expect("location error");
    let a = &a[..a.len()-2];
    let a = PathBuf::from(a);
a
}
fn main(){

    let path1 = read();
    let path2 = read();
    
    replace(path1,path2);

}