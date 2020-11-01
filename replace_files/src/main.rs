use std::{fs,env,io};
use std::path::PathBuf;
use std::io::Result as IOResult;


fn replace(a:PathBuf,b:PathBuf,a_files:Vec<PathBuf>,b_files:Vec<PathBuf>) {
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
fn readdir(path:PathBuf) ->IOResult<Vec<PathBuf>> {
    let entr = fs::read_dir(&path)?
        .map(|res|res.map(|e|e.path()))
        .collect::<Result<Vec<_>,io::Error>>()?;
    Ok(entr)

}
fn start(a:&String) -> (PathBuf,Vec<PathBuf>){
    let a = PathBuf::from(a);
    if a.is_file() == true {

    }

    let b = readdir(a.clone()).unwrap();

(a,b)
}
fn main(){
    let args: Vec<String> = env::args().collect();
    let path1 = start(&args[1]);
    let path2 = start(&args[2]);
    
    replace(path1.0,path2.0,path1.1,path2.1);
}
