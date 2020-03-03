use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    // Receives a single datagram message on the socket. If `buf` is too small to hold
    // the message, it will be cut off.
    loop {
        let mut buf = [0; 10];
        let (amt, src) = socket.recv_from(&mut buf)?;

        // Redeclare `buf` as slice of the received data and send reverse data back to origin.
        let buf = &mut buf[..amt];
        println!("receive {:?}", String::from_utf8(buf.to_vec()).unwrap());
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
    
    Ok(())
}