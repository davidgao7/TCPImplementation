extern crate tun_tap;  // Declares the use of the tun_tap crate, which provides bindings for creating and managing TUN/TAP interfaces.
extern crate hex; // hexdecimal encoding/decoding

use std::io;
use hex_fmt::HexFmt;

fn main() -> io::Result<()> {
    // see if tun_tap working
    /*
    * Ok(()) : Success
    * Err(io::Error) : an error from the standard I/O library
    * */

    print!("======run till here 0=============\n");
    // create a TUN interface
    // network interface name: `tun0`
    // `tun_tap::Mode::Tun`: specifies that this is a **TUN interface** (layer 3 - IP packets)
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    print!("======run till here 1=============\n");

    // create a new interface
    // this buffer will hold the incoming packet data from the TUN interface
    let mut buf = [0u8; 1504]; // create a buffer of size 1504 bytes
    print!("======run till here 2=============\n");

    // receiving data
    // this is a **blocking operation**, the program will wait until a packet is received
    let nbytes = nic.recv(&mut buf[..])?;  // Receives a packet from the interface, return #bytes copied into the buffer
    print!("======run till here 3=============\n");

    // nbytes: the number of bytes read into buffer
    // &buf[..nbytes] : a slice of the buffer containing only the received data
    // {:x} : attempts to format the buffer slice as hexadecimal
    print!("======run till here 4=============\n");
    eprintln!("read {} bytes: {}", nbytes, HexFmt(&buf[..nbytes]));

    Ok(()) // success
}
