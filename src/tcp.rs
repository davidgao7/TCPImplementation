use std::io;

pub enum State {
    Closed,
    Listen,
    SynRcvd, // SYN Recieved
    Estab,   // Established
}

impl Default for State {
    fn default() -> Self {
        //   State::Closed
        //   starting out with listen
        State::Listen
    }
}

impl State {
    pub fn on_packet<'a>(
        &mut self,
        nic: &mut tun_tap::Iface,
        iph: etherparse::Ipv4HeaderSlice<'a>,
        tcph: etherparse::TcpHeaderSlice<'a>,
        data: &'a [u8],
    ) -> io::Result<usize> {
        let mut buf = [0u8; 1500];
        let mut unwritten = &mut buf[..];
        match *self {
            // if we recieved a packet

            // check the state we're in
            State::Closed => {
                return Ok(0);
            }
            State::Listen => {
                if !tcph.syn() {
                    // only expetcted SYN packet
                    return;
                }

                // need to establish a connection
                let mut syn_ack = etherparse::TcpHeader::new(
                    tcph.destination_port(),
                    tcph.source_port(),
                    unimplemented!(),
                    unimplemented!(),
                );
                syn_ack.syn = true;
                syn_ack.ack = true;

                // create a ipv4 header
                let mut ip = etherparse::Ipv4Header::new(
                    syn_ack.slice().len(),
                    64,
                    etherparse::IpTrafficClass::Tcp,
                    [ 
                        iph.destination()[0], 
                    iph.destination()[1],
                    iph.destination()[2],
                    iph.destination()[3],
                    ],
                    iph.source_addr(),
                );

                // wite out the headers
                let unwritten = {
                    let mut unwritten = &mut buf[..];
                    ip.write(unwritten);
                    syn_ack.write(unwritten);
                    unwritten.len()
                };
                nic.send(&buf[..unwritten])
            }
        }
        eprintln!(
            "{}:{} -> {}:{} {}b of tcp",
            iph.source_addr(),
            tcph.source_port(),
            iph.destination_addr(),
            tcph.destination_port(),
            data.len(),
        )
    }
}
