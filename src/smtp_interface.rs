use tokio::codec::LinesCodec;
use tokio::codec::Framed;
use tokio::net::TcpStream;


pub struct SMTPEnvelope {
    pub reverse_path: String,
    pub forward_paths: Vec<String>,
}

pub struct Connecting {
    pub framed_socket: Framed<TcpStream, LinesCodec>,
}

pub struct Connected {
    pub framed_socket: Framed<TcpStream, LinesCodec>,
}

pub struct SessionStarted {
    pub framed_socket: Framed<TcpStream, LinesCodec>,
}

pub struct TransactionStarted {
    pub framed_socket: Framed<TcpStream, LinesCodec>,
    pub envelope: SMTPEnvelope,
}

pub struct TransactionReady {
    pub framed_socket: Framed<TcpStream, LinesCodec>,
    pub envelope: SMTPEnvelope,
}

pub struct DataMode {
    pub framed_socket: Framed<TcpStream, LinesCodec>,
    pub envelope: SMTPEnvelope,
}

pub struct Disconnected {

}

pub enum ServerState {
    Connecting(Connecting),
    Connected(Connected),
    SessionStarted(SessionStarted),
    TransactionStarted(TransactionStarted),
    TransactionReady(TransactionReady),
    DataMode(DataMode),
    Disconnected(Disconnected),
}