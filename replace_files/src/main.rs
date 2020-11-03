use std::{fs,env,io};
use std::path::PathBuf;
use std::io::Result as IOResult;

fn replace_file(a:PathBuf,b:PathBuf) {
    let mut temp = b
        .clone()
        .join(".tmp");
    let temp = PathBuf::from(temp);
    fs::copy(&a,&temp);
    fs::remove_file(&a);
    fs::copy(&b,&a);
    fs::rename(&temp,&a);
}
fn replace_dir(a:PathBuf,b:PathBuf,a_files:Vec<PathBuf>,b_files:Vec<PathBuf>) {
    let mut temp = a
        .clone()
        .join(".tmp");
    fs::create_dir(&temp);
    for x in 0..a_files.len(){
        let c = a_files.get(x).unwrap();
        fs::copy(&c,&temp);
        fs::remove_file(&c);
        }
    for y in 0..b_files.len(){
        let d = b_files.get(y).unwrap();
        fs::copy(&d,&a);
        fs::remove_file(&d);
    }
    fs::remove_dir_all(temp);

}
fn readdir(path:PathBuf) ->IOResult<Vec<PathBuf>> {
    let entr = fs::read_dir(&path)?
        .map(|res|res.map(|e|e.path()))
        .collect::<Result<Vec<_>,io::Error>>()?;
    Ok(entr)

}
fn start(a:&String) -> (PathBuf,Vec<PathBuf>){
    let a = PathBuf::from(a);
    let b = readdir(a.clone()).unwrap();

(a,b)
}
fn main(){
    let args: Vec<String> = env::args().collect();
    let path1 = start(&args[1]);
    let path2 = start(&args[2]);
    if path1.0.is_file() == true || path2.0.is_file() == true {
        replace_file(path1.0,path2.0);
    } else {
        replace_dir(path1.0,path2.0,path1.1,path2.1);
    }
}
