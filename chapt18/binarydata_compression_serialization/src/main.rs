use std::io;
use std::io::prelude::*;
use std::fs::File;


fn test_binary_data() -> io::Result<()> {
    use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
    let mut reader = File::open("new_file.txt")?;
    let mut writer = File::create("test_binary_data.txt")?;

    let n = reader.read_u32::<LittleEndian>()?;
    println!("{}", n);
    writer.write_i64::<LittleEndian>(n as i64)?;
    Ok(())


}

fn test_compression() -> io::Result<()> {
    use flate2::read::GzDecoder;
    let file = File::open("access.log.gz")?;
    let mut gzip_reader = GzDecoder::new(file);
    let mut s = String::new();
    gzip_reader.read_to_string(&mut s)?;
    println!("gizp reader: {}", s);
    Ok(())
}


fn test_serde() -> io::Result<()> {
    use std::collections::HashMap;
    use serde::{Serialize, Deserialize};

    type RoomId = String; // echo room has a unique name
    type RoomExits = Vec<(char, RoomId)>; // ...and a list of exits
    type RoomMap = HashMap<RoomId, RoomExits>; // room names and exits, simple

    // create a simple map.
    let mut map = RoomMap::new();
    map.insert("Cobble Crawl".to_string(),
        vec![('W', "Debris Room".to_string())]);
    map.insert("Debris Room".to_string(),
        vec![('E', "Cobble Crawl".to_string()),
            ('W', "Sloping Canyon".to_string())]);
    serde_json::to_writer(&mut std::io::stdout(), &map)?;
    println!();
    #[derive(Serialize, Deserialize)]
    struct Player {
        location: String,
        items: Vec<String>,
        health: u32
    }
    let player = Player {
        location : "Cobble Crawl".to_string(),
        items: vec!["a wand".to_string()],
        health: 3

    };
    serde_json::to_writer(&mut std::io::stdout(), &player)?;
    println!();
    Ok(())
}

fn main() {
    let result = test_binary_data();
    if let Err(err) = result {
        eprintln!("test binary data:{}", err);
        std::process::exit(1);
    }
    let result = test_compression();
    if let Err(err) = result {
        eprintln!("test compression: {}", err);
        std::process::exit(1);
    }
    let result = test_serde();
    if let Err(err) = result {
        eprintln!("test serde: {}", err);
        std::process::exit(1);
    }

}
