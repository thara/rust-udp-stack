use std::io;

fn main() -> io::Result<()> {
    let sock = unsafe {
        match libc::socket(
            libc::AF_PACKET,
            libc::SOCK_RAW,
            (libc::ETH_P_ALL as u16).to_be() as _,
        ) {
            -1 => Err(io::Error::last_os_error()),
            fd => Ok(fd),
        }
    }?;
    println!("sock: {}", sock);

    let mut buf = [0u8; 1024];
    loop {
        let len = unsafe {
            libc::recv(
                sock,
                (&mut buf[..]).as_mut_ptr() as *mut libc::c_void,
                buf.len(),
                0,
            )
        };
        println!("len: {}", len);
    }
}
