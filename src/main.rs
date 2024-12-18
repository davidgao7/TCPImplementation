extern crate tun_tap;
use std::io;

fn main() -> io::Result<()> {
    // see if tun_tap working

    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun).expect("failed to cr")?;

    // create a new interface
    let mut buf = [0u8; 1504];

    let nbytes = nic.recv(&mut buf[..])?;
    eprintln!("read {} bytes: {:x}", nbytes, &buf[..nbytes]);

    Ok(())
}
