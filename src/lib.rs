extern crate capnp;


use std::io::net::ip::{SocketAddr,Ipv4Addr};
use std::io::net::udp::{UdpStream, UdpSocket};

use capnp::{MessageBuilder, MessageReader, MallocMessageBuilder, ReaderOptions, serialize_packed};
use proto_capnp::command;

pub mod proto_capnp;

pub struct Conn {
    socket: UdpSocket,
    addr_to_send_to: Option<SocketAddr>,
}

impl Conn {
    pub fn new_client(addr: SocketAddr) -> Conn {
        Conn {
            socket: UdpSocket::bind(SocketAddr {ip: Ipv4Addr(0,0,0,0), port: 0} ).ok().expect("Bind failure"),
            addr_to_send_to: Some(addr),
        }
    }

    pub fn new_server(addr: SocketAddr) -> Conn {
        Conn {
            socket: UdpSocket::bind(addr).ok().expect("Bind failure"),
            addr_to_send_to: None,
        }
    }

    pub fn send_move_cube(&mut self, dx: f32, dy: f32, dz: f32) {
        let mut msg = MallocMessageBuilder::new_default();
        {
            let cmd = msg.init_root::<command::Builder>();

            cmd.set_id(1);
            let mc = cmd.init_move_cube();
            mc.set_d_x(dx);
            mc.set_d_y(dy);
            mc.set_d_z(dz);
        }

        match self.addr_to_send_to {
            Some(mut addr) => {
                serialize_packed::write_packed_message_unbuffered(&mut self.socket.clone().connect(addr), &msg);
            },
            None => println!("PORBLEM? send_move_cube doesn't know who to send to!")
        }
    }

    pub fn receive_move_cube(&mut self) -> (f32, f32, f32) {
        let mut buffer = [0, ..4096];
        let (size, from) = self.socket.recv_from(buffer.as_mut_slice()).unwrap();
        let mut buf = std::io::BufReader::new(buffer.as_slice().slice_to(size));
        let mut msg = serialize_packed::new_reader_unbuffered(&mut buf, ReaderOptions::new()).ok().expect("Reader creation failed");

        let cmd = msg.get_root::<command::Reader>();
        match cmd.which() {
            Some(command::MoveCube(cube)) => {
                (cube.get_d_x(), cube.get_d_y(), cube.get_d_z())
            }
            _ => panic!("Invalid packet")
        }
    }
}

#[test]
fn move_cube_test() {
    let mut conn = Conn::new(SocketAddr{ip: Ipv4Addr(127, 0, 0, 1), port: 34000});

    conn.sendMoveCube(10f32, 5f32, 0f32);
}
