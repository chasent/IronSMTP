use std::io::Error;
use std::io::prelude::*;
use tokio::net::TcpStream;
use std::io::BufReader;
use std::{thread, time};

pub enum ClientCommand {
    Hello       { domain: String        },  // HELO <SP> <domain> <CRLF>
    Mail        { reverse_path: String  },  // MAIL <SP> FROM:<reverse-path> <CRLF>
    Recipient   { forward_path: String  },  // RCPT <SP> TO:<forward-path> <CRLF>
    Data,                                   // DATA <CRLF>
    Reset,                                  // RSET <CRLF>
    Send        { reverse_path: String  },  // SEND <SP> FROM:<reverse-path> <CRLF>
    SendOrMail  { reverse_path: String  },  // SOML <SP> FROM:<reverse-path> <CRLF>
    SendAndMail { reverse_path: String  },  // SAML <SP> FROM:<reverse-path> <CRLF>
    Verify      { arg: String           },  // VRFY <SP> <string> <CRLF>
    Expand      { arg: String           },  // EXPN <SP> <string> <CRLF>
    Help        { arg: String           },  // HELP [<SP> <string>] <CRLF>
    NoOp,                                   // NOOP <CRLF>
    Quit,                                   // QUIT <CRLF>
    Turn,                                   // TURN <CRLF>
    Invalid     { invalid_cmd: String   },
}


pub fn read_client_data(stream: &TcpStream) -> Result<String, Error> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    while !line.ends_with("\r\n.\r\n") {
        let readOp = reader.read_line(&mut line);
        match readOp {
            Ok(bytes_read) => {
                println!("Read {} bytes of DATA from client.", bytes_read);
            },
            Err(err) => {
                return Result::Err(err);
            },
        }
    }

    return Result::Ok(line);
}



fn read_from_client(stream: &TcpStream) -> Result<String, Error> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();

    while !line.ends_with("\r\n") {
        thread::sleep(time::Duration::from_millis(500));

        let read_op = reader.read_line(&mut line);
        match read_op {
            Ok(bytes_read) => {
                println!("Read {} bytes read from client.", bytes_read);
            },
            Err(err) => {
                return Result::Err(err);
            },
        }
    }

    Result::Ok(line)
}


fn parse_command(cmd: String) -> ClientCommand {
    if cmd.len() < 4 {
        return ClientCommand::Invalid { invalid_cmd: cmd };
    }

    match &cmd.to_uppercase()[..4] {
        "HELO"  => ClientCommand::Hello         { domain: cmd[5..].trim_right().to_string()         },
        "MAIL"  => ClientCommand::Mail          { reverse_path: cmd[10..].trim_right().to_string()  },
        "RCPT"  => ClientCommand::Recipient     { forward_path: cmd[8..].trim_right().to_string()   },
        "DATA"  => ClientCommand::Data,
        "RSET"  => ClientCommand::Reset,
        // "SEND"  => ClientCommand::Send          { reverse_path: cmd[10..].trim_right().to_string()  }, 
        // "SOML"  => ClientCommand::SendOrMail    { reverse_path: cmd[10..].trim_right().to_string()  }, 
        // "SAML"  => ClientCommand::SendAndMail   { reverse_path: cmd[10..].trim_right().to_string()  },
        // "VRFY"  => ClientCommand::Verify        { arg: cmd[5..].trim_right().to_string()            }, 
        // "EXPN"  => ClientCommand::Expand        { arg: cmd[5..].trim_right().to_string()            }, 
        // "HELP"  => ClientCommand::Help          { arg: cmd[5..].trim_right().to_string()            },
        "NOOP"  => ClientCommand::NoOp, 
        "QUIT"  => ClientCommand::Quit, 
        // "TURN"  => ClientCommand::Turn,
        _       => ClientCommand::Invalid       { invalid_cmd: cmd                                  },
    }
}


pub fn read_client_command(stream: &TcpStream) -> Result<ClientCommand, Error> {
    read_from_client(stream)
        .map(|cmd| {
            println!("CLIENT: {}", cmd.trim());
            cmd
        })
        .map(parse_command)
}