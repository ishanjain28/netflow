use netflow_zmq::netflow_v9::*;
use std::{io::Result as IoResult, net::UdpSocket, sync::mpsc, thread, time::SystemTime};
use zmq::Message;

fn main() -> IoResult<()> {
    let (sender, receiver) = mpsc::channel();

    let handle = thread::spawn(move || {
        let socket =
            UdpSocket::bind("0.0.0.0:9995").expect("could not start listener on port 9995");
        let mut parser = Parser::new();
        let mut buffer = [0; 1 << 12];

        loop {
            let (n, src) = socket
                .recv_from(&mut buffer)
                .expect("error in reading input");

            let t = SystemTime::now();

            let packet = parser.parse(&buffer[..n]);

            println!(
                "read {} bytes from {} and parsed packet in {} nanos",
                n,
                src,
                t.elapsed()
                    .expect("error in reading elapsed time")
                    .as_nanos()
            );

            sender
                .send(packet)
                .expect("error in sending packet to mpsc");
        }
    });

    let ctx = zmq::Context::new();

    let socket = ctx.socket(zmq::PUB).unwrap();
    socket.bind("tcp://0.0.0.0:9996")?;

    for packet in receiver {
        println!("received packet: {:?}", packet);

        socket.send_msg(Message {}, flags)
    }

    handle.join().unwrap();

    //    for stream in socket.incoming() {
    //        let mut stream = match stream {
    //            Ok(v) => v,
    //            Err(e) => {
    //                eprintln!("error in accepting connection: {:?}", e);
    //                continue;
    //            }
    //        };
    //
    //    }

    Ok(())
}
