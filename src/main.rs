extern crate tun_tap; // Declares the use of the tun_tap crate, which provides bindings for creating and managing TUN/TAP interfaces. // hexdecimal encoding/decoding
use std::collections::HashMap;
use std::io;
use std::net::Ipv4Addr;

mod tcp;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
struct Quad {
    src: (Ipv4Addr, u16),
    dst: (Ipv4Addr, u16),
}

fn main() -> io::Result<()> {
    // see if tun_tap working
    /*
     * Ok(()) : Success
     * Err(io::Error) : an error from the standard I/O library
     * */

    let mut connections: HashMap<Quad, tcp::State> = Default::default();
    //print!("======run till here 0=============\n");
    // create a TUN interface
    // network interface name: `tun0`
    // `tun_tap::Mode::Tun`: specifies that this is a **TUN interface** (layer 3 - IP packets)
    // if successful, the program sets up `tun0` as a virtual device for capturing or injecting
    // packets
    let nic = tun_tap::Iface::new("tun0", tun_tap::Mode::Tun)?;
    eprintln!("Waiting for packets on tun0...");
    //print!("======run till here 1=============\n");

    // create a new interface
    // this buffer will hold the incoming packet data from the TUN interface
    let mut buf = [0u8; 1504]; // create a buffer of size 1504 bytes
                               //print!("======run till here 2=============\n");

    // receiving data
    // this is a **blocking operation**, the program will wait until a packet is received
    // the process repeats for every incoming packet, continuously listening for data on `tun0`
    loop {
        let nbytes = nic.recv(&mut buf[..])?; // Receives a packet from the interface(when a packet arrives),
                                              // return #bytes copied into the buffer
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
        // enthernet frame we got,
        // link level protocal
        let _eth_flags = u16::from_be_bytes([buf[0], buf[1]]); // byte 0-1: represent flags (interpreted as a u16 integer in big-endian order)
        let eth_proto = u16::from_be_bytes([buf[2], buf[3]]); // protocol type (e.g. IPv4, IPv6)

        // NOTE: filter anything that is not ipv4 packet
        if eth_proto != 0x0800 {
            // no ipv4
            continue;
        }

        match etherparse::Ipv4HeaderSlice::from_slice(&buf[4..nbytes]) {
            Ok(ip_header) => {
                // nbytes: the number of bytes read into buffer
                // &buf[..nbytes] : a slice of the buffer containing only the received data

                // {:x} : attempts to format the buffer slice as hexadecimal
                //print!("======run till here 4=============\n");
                // NOTE: get proto type : 86dd , which is the enthertype that indicates the payload of an
                // enthernet frame contains an IPV6 (internet protocol version 6) packet

                // ip level protocal, should be set to tcp
                let src = ip_header.source_addr();
                let dst = ip_header.destination_addr();
                let protocal = ip_header.protocol();

                eprintln!(
                    "Packet captured on tun0: protocol {:02x}, length {}",
                    protocal, nbytes
                );

                // NOTE: only processes TCP packets
                if protocal != 0x06 {
                    // not tcp
                    continue;
                }

                match etherparse::TcpHeaderSlice::from_slice(
                    &buf[4 + ip_header.slice().len()..nbytes],
                ) {
                    Ok(tcp_header) => {
                        let datai = 4 + ip_header.slice().len() + tcp_header.slice().len();
                        connections
                            .entry(
                                // if there's a quad it will show, or just create one
                                Quad {
                                    src: (src, tcp_header.source_port()),
                                    dst: (dst, tcp_header.destination_port()),
                                },
                            )
                            .or_default() // after that : a mutable reference to the state
                            .on_packet(ip_header, tcp_header, &buf[datai..]);
                        eprintln!(
                            "TCP packet: {} -> {} {}b of tcp to port {}",
                            src,
                            dst,
                            tcp_header.slice().len(),
                            tcp_header.destination_port()
                        );
                    }
                    Err(e) => {
                        eprintln!("ignoring weird tcp packet: {:?}", e)
                    }
                }
            }
            Err(e) => {
                eprintln!("ignoring weird packet {:?}", e)
            }
        }
    }
    //Ok(())
}
