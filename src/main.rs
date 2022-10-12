use std::time::Duration;

fn main() {
    loop {
        let mut serial = match serialport::new("/dev/ttyUSB0", 115_200)
            .timeout(Duration::from_secs(2))
            .open()
        {
            Ok(s) => s,
            Err(e) => {
                println!("bad: {:?}", e);
                continue;
            }
        };

        let mut buf = [0u8; 4096];
        if let Ok(count) = serial.read(&mut buf) {
            let buf = &buf[..count];
            //println!("read {count} bytes");
            if let Ok(s) = std::str::from_utf8(buf) {
                print!("{s}");
            }
        }
    }
}
