extern crate tun_tap;  // Declares the use of the tun_tap crate, which provides bindings for creating and managing TUN/TAP interfaces.
extern crate hex; // hexdecimal encoding/decoding

use std::io;
//use hex_fmt::HexFmt;

fn main() -> io::Result<()> {
    // see if tun_tap working
    /*
    * Ok(()) : Success
    * Err(io::Error) : an error from the standard I/O library
    * */

    //print!("======run till here 0=============\n");
    // create a TUN interface
    // network interface name: `tun0`
    // `tun_tap::Mode::Tun`: specifies that this is a **TUN interface** (layer 3 - IP packets)
    // if successful, the program sets up `tun0` as a virtual device for capturing or injecting
    // packets
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    //print!("======run till here 1=============\n");

    // create a new interface
    // this buffer will hold the incoming packet data from the TUN interface
    let mut buf = [0u8; 1504]; // create a buffer of size 1504 bytes
    //print!("======run till here 2=============\n");

    // receiving data
    // this is a **blocking operation**, the program will wait until a packet is received
    // the process repeats for every incoming packet, continuously listening for data on `tun0`
    loop{
        let nbytes = nic.recv(&mut buf[..])?;  // Receives a packet from the interface(when a packet arrives), return #bytes copied into the buffer
        //print!("======run till here 3=============\n");

        /*
        * u16::from_be_bytes
        * constructs a `u16` integer from a 2-element byte array interpreted in big-endian
        * (network) byte order. This means the most significant byte is at the first position
        * of the array
        *
        * Parameters:
        * `bytes`: a `[u8; 2] array representing the bytes in big-endian order
        *
        * Output
        * returns a `u16` integer composed from the provided byte array*/
        // the first 4 bytes of the buffer ar einterpreted as metadata
        let flags = u16::from_be_bytes([buf[0], buf[1]]); // byte 0-1: represent flags (interpreted as a u16 integer in big-endian order)
        let proto = u16::from_be_bytes([buf[2], buf[3]]); // protocol type (e.g. IPv4, IPv6)

        // nbytes: the number of bytes read into buffer
        // &buf[..nbytes] : a slice of the buffer containing only the received data
        // {:x} : attempts to format the buffer slice as hexadecimal
        //print!("======run till here 4=============\n");
        eprintln!("read {} bytes (flags: {:x}, proto: {:x}: {:x?})", nbytes - 4, flags, proto, &buf[4..nbytes]);
    }
}
