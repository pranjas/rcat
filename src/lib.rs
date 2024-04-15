use std::io::{Read, Seek, SeekFrom};
use std::fs::File;

//There's no close method on File since as soon as the reference
//is dropped, file would be closed.
pub fn read_content(fname: &str, buffer :&mut[u8], offset :u64)->Option<usize> {
    let mut file = match File::open(fname) {
        Ok(_f) => _f,
        Err(e) => { //if there's an error we want to return early.
            eprintln!("unable to open file {}", e);
            return None; //without return, we'll have to give a File from here.
        }
    };

    match file.seek(SeekFrom::Start(offset)) {
        Ok(off) => dbg!("Seeked to file offset = {}", off) , //if seek is Ok then good.
        Err(e) => {
            eprintln!("error seeking to offset {}, err = {}", offset, e);
            return None; //couldn't seek so just give up.
        }
    };
    match file.read(buffer) {
        Ok(nbytes) => Some(nbytes),
        Err(_) => None
    }
}