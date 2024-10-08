use chess_lib::board::pieces::Color;
use ggez::{GameError, GameResult};
use rmp_serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::{
    io::{BufWriter, Write},
    net::{TcpListener, TcpStream},
    sync::Arc,
    thread,
    time::Duration,
};

#[derive(Debug, PartialEq)]
pub enum MultiplayerStatus {
    Server,
    Client,
}

pub struct Connection {
    pub multiplayer_status: MultiplayerStatus,
    pub local_color: Color,
    stream: BufWriter<TcpStream>,
}

#[derive(Debug)]
pub enum ReadError {
    IO(std::io::Error),
    Decode(rmp_serde::decode::Error),
}
impl From<std::io::Error> for ReadError {
    fn from(err: std::io::Error) -> ReadError {
        ReadError::IO(err)
    }
}
impl From<rmp_serde::decode::Error> for ReadError {
    fn from(err: rmp_serde::decode::Error) -> ReadError {
        ReadError::Decode(err)
    }
}

#[derive(Debug)]
pub enum WriteError {
    IO(std::io::Error),
    Encode(rmp_serde::encode::Error),
}
impl From<std::io::Error> for WriteError {
    fn from(err: std::io::Error) -> WriteError {
        WriteError::IO(err)
    }
}
impl From<rmp_serde::encode::Error> for WriteError {
    fn from(err: rmp_serde::encode::Error) -> WriteError {
        WriteError::Encode(err)
    }
}

impl From<ReadError> for GameError {
    fn from(err: ReadError) -> GameError {
        match err {
            ReadError::IO(e) => GameError::IOError(Arc::new(e)),
            ReadError::Decode(e) => GameError::CustomError(e.to_string()),
        }
    }
}

impl From<WriteError> for GameError {
    fn from(err: WriteError) -> GameError {
        match err {
            WriteError::IO(e) => GameError::IOError(Arc::new(e)),
            WriteError::Encode(e) => GameError::CustomError(e.to_string()),
        }
    }
}

impl Connection {
    pub fn server(port: u16) -> std::io::Result<Connection> {
        let listener = TcpListener::bind(("0.0.0.0", port))?;
        let (stream, _addr) = listener.accept()?;
        stream.set_nonblocking(true)?;

        Ok(Connection {
            multiplayer_status: MultiplayerStatus::Server,
            stream: BufWriter::new(stream),
            local_color: Color::EMPTY,
        })
    }

    pub fn client(addr: &str, port: u16) -> std::io::Result<Connection> {
        let stream = TcpStream::connect((addr, port))?;
        stream.set_nonblocking(true)?;

        Ok(Connection {
            multiplayer_status: MultiplayerStatus::Client,
            stream: BufWriter::new(stream),
            local_color: Color::EMPTY,
        })
    }

    pub fn write<T: Serialize + std::fmt::Debug>(&mut self, packet: T) -> Result<(), WriteError> {
        println!("Sending: {:?}", packet);
        packet.serialize(&mut Serializer::new(&mut self.stream))?;
        self.stream.flush()?;
        Ok(())
    }

    pub fn read<T: for<'a> Deserialize<'a> + std::fmt::Debug>(&mut self) -> Result<T, ReadError> {
        let mut de = Deserializer::new(self.stream.get_mut());
        let packet = T::deserialize(&mut de)?;
        println!("Receiving: {:?}", packet);
        Ok(packet)
    }

    pub fn read_block<T: for<'a> Deserialize<'a> + std::fmt::Debug>(
        &mut self,
    ) -> Result<T, ReadError> {
        loop {
            let packet = self.read::<T>();
            if let Ok(p) = packet {
                return Ok(p);
            };
            thread::sleep(Duration::from_millis(20));
        }
    }
}
