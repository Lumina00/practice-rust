use std::{fs,io};
use std::path::PathBuf;
use std::ffi::OsString;

fn scan(path:PathBuf) -> io::Result<Vec<PathBuf>> {
    let entr = fs::read_dir(&path)?
        .map(|res|res.map(|e|e.path()))
        .collect::<Result<Vec<_>,io::Error>>()?;

    Ok(entr)
}
fn change(a:io::Result<Vec<PathBuf>>) -> Vec<String>{
    let a = a
            .unwrap()
            .into_iter()
            .map(PathBuf::into_os_string)
            .map(OsString::into_string)
            .collect::<Result<_,_>>()
            .unwrap();
a
}
fn get_size(a:&io::Result<Vec<PathBuf>>) ->Vec<u64>{
    let a = a.as_ref().unwrap();
    let mut z = Vec::new();
    for (i,j) in a.iter().enumerate(){
        let b = &a[i];
        let b = fs::metadata(b)
            .expect("could not get metadata");
        let b = b.len();
        z.push(b);
    }
z    
}
fn do_match<T: PartialEq>(a: &Vec<T>, b: &Vec<T>) -> bool{
    let matching = a
        .iter()
        .zip(b.iter())
        .filter(|&(a,b) |  a==b)
        .count();
    matching == a.len() && matching == b.len()
}
fn check(a: &[String], b: &[String]) -> Vec<String>{
   let mut c:Vec<String> = a
       .iter()
       .chain(b.iter())
       .cloned()
       .collect();
    
    let mut i = 0;
    
    while let Some(s) = c.get(i){
        i += 1;
        
        if c[i..].contains(s){
            let s = s.clone();
            c.retain(|x| x!= &s);
            i -= 1;
        }
    }
c
        }
fn started() -> (Vec<String>,Vec<u64>) {
    let a = read();
    let a = scan(a);
    let a_size = get_size(&a);
    let a = change(a);
    (a,a_size)
}
fn read() -> PathBuf{
    let mut a = String::new();
    io::stdin().read_line(&mut a)
        .expect("location error");
    let a = &a[..a.len()-1];
    let a = PathBuf::from(a);
a
}
fn main(){
    let buf1 = started();
    let buf2 = started();
    let path1 = buf1.0;
    let path1_size = buf1.1;
    let path2 = buf2.0;
    let path2_size = buf2.1;

    let sum = do_match(&path1,&path2);
    let mut result = Vec::new();
    match sum {
        true => println!("same"),
        false => result= check(&path1,&path2),
        _=> panic!("what makes something bad"),
    }
    println!("{:?}",result);
}
