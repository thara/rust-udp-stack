use std::net::UdpSocket;

const BUFFER_SIZE: usize = 1024;

fn main() -> std::io::Result<()> {
    loop {
        let socket = UdpSocket::bind("127.0.0.1:34254")?;

        let mut buf = [0; BUFFER_SIZE];
        let (n, src) = socket.recv_from(&mut buf)?;

        let buf = &mut buf[..n];
        buf.reverse();
        socket.send_to(buf, &src)?;
    }
}
