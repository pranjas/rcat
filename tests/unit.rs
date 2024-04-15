use rcat;
use std::vec::Vec;
use ascii::AsciiString;
use std::io::Write;

#[test]
fn test_non_existent_file() {
    assert_eq!(rcat::read_content("a_non_existent_file", &mut Vec::<u8>::with_capacity(0), 0),
                None)
}

#[test]
fn test_existent_file() {
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    let mut buffer = vec![0_u8; 0];
    //take the path out.
    //we've not written anything to file, so this should
    //not give as an error but a value of 0 when reading from it.
    let (_, path) = temp_file.keep().unwrap();
    let path = path.to_str().unwrap();
    assert_eq!(rcat::read_content(&path, &mut buffer, 0), Some(0));
}

#[test]
fn test_file_contents_4KPage() {
    let mut temp_file = tempfile::NamedTempFile::new().unwrap();
    let test_content = "This is a test content";
    
    //allocate a buffer and initialize to 0.
    //a Vec::<u8>::with_capacity(4096) won't actually allocate
    //memory, such that when the buffer is passed to read file
    //it would return nothing since buffer has no len.
    let mut buffer = vec![0_u8; 4096];

    //just like println! write! requires a formatter
    //when writing variables.
    let _ = write!(temp_file, "{}", test_content);
    let _ = temp_file.as_file_mut().sync_all(); //need to be sure if disk has contents.
    //keep the temporary file around so we can read back data
    //from it.
    let (_, path_name) = temp_file.keep().unwrap(); //this also closes file.

    let fname :&str = path_name.to_str().unwrap();
    
    let bytes_read = rcat::read_content(&fname, &mut buffer, 0).unwrap();
    let content = AsciiString::from_ascii(&buffer[..bytes_read]).unwrap();

    assert_eq!(content.as_str(), test_content);
}
