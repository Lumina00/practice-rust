use std::{fs,io};
use std::path::PathBuf;
use std::ffi::OsString;

fn replace(a:PathBuf,b:PathBuf) {
    let mut temp = b
        .clone()
        .into_os_string()
        .into_string()
        .unwrap();
    temp.push_str("wt");
    let temp = PathBuf::from(temp);
    println!("{:?} {:?}",&a.display(),&b.display());
    println!("{}",&temp.display());
    fs::copy(&a,&temp)
        .expect("error 1");
    fs::remove_file(&a)
        .expect("error 2");
    fs::copy(&b,&a)
        .expect("error 3");
    fs::rename(&temp,&a)
        .expect("error 4");
    
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