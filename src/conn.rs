use std::io::{BufferedWriter, IoResult, TcpStream};
use data::Message;

pub enum Connection {
    TcpConn(TcpStream),
}

pub fn connect(host: &str, port: u16) -> IoResult<Connection> {
    let socket = try!(TcpStream::connect(host, port));
    Ok(TcpConn(socket))
}

fn send_internal(conn: &Connection, msg: &str) -> IoResult<()> {
    match conn {
        &TcpConn(ref tcp) => {
            let mut writer = BufferedWriter::new(tcp.clone());
            writer.write_str(msg)
        }
    }
}

pub fn send(conn: &Connection, msg: Message) -> IoResult<()> {
    let mut send = msg.command.to_string();
    send.push_str(" ");
    send.push_str(msg.args.init().connect(" ").as_slice());
    send.push_str(" :");
    send.push_str(*msg.args.last().unwrap());
    send.push_str("\r\n");
    send_internal(conn, send.as_slice())
}