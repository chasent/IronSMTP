
// use tokio::net::TcpListener;
// use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// mod client_reader;
// use client_reader::*;
// use client_reader::ClientCommand;

// mod client_writer;
// use client_writer::*;

mod smtp_interface;
use tokio::prelude::future::result;
use smtp_interface::*;


//fn main() {
    // let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 12345);

    // let server_state = TcpListener::bind(&addr).expect("unable to bind TCP listener");

    // // // let server = server_state.incoming()
    // // //     .map_err(|e| eprintln!("accept failed = {:?}", e))
    // // //     .for_each(|sock| {
    // // //         let (reader, writer) = sock.framed

    // // //         let b = reader.for_each(|frame| {

    // // //             panic!();
    // // //         });

    // // //         //tokio::spawn(process(sock))
    // // //         Ok(())
    // // //     });

    // // //     // Start the runtime and spin up the server
    // // //     tokio::run(server);

        // .map(|listener| {
        //     let a = listener.;

        //     return "yo";
        //     //ServerState::AwaitingConnection (AwaitingConnection { listener: listener })
        // });

    // while let Ok(ss) = server_state {
    //     match ss {
    //         ServerState::AwaitingConnection(state) => {
    //             server_state = handle_awaiting_connection(state);
    //         },
    //         ServerState::Connected(state) => {
    //             server_state = handle_connected(state);
    //         },
    //         ServerState::SessionStarted(state) => {
    //             server_state = handle_session_started(state);
    //         },
    //         ServerState::TransactionStarted(state) => {
    //             server_state = handle_transaction_started(state);
    //         },
    //         ServerState::TransactionReady(state) => {
    //             server_state = handle_transation_ready(state);
    //         },
    //         ServerState::DataMode(state) => {
    //             server_state = handle_data_mode(state);
    //         },
    //     }
    // }
//}

// HELO client.example.com
// MAIL FROM:<mail@samlogic.com>
// RCPT TO:<john@mail.com>
// DATA
// <The message data (body text, subject, e-mail header, attachments etc) is sent>
// .
// QUIT


// S: 220 smtp.server.com Simple Mail Transfer Service Ready
// C: HELO client.example.com
// S: 250 Hello client.example.com
// C: MAIL FROM:<mail@samlogic.com>
// S: 250 OK
// C: RCPT TO:<john@mail.com>
// S: 250 OK
// C: DATA
// S: 354 Send message content; end with <CRLF>.<CRLF>
// C: <The message data (body text, subject, e-mail header, attachments etc) is sent>
// C: .
// S: 250 OK, message accepted for delivery: queued as 12345
// C: QUIT
// S: 221 Bye








extern crate tokio;
extern crate tokio_codec;
#[macro_use]
extern crate futures;

use tokio::prelude::*;
use tokio::io;
use tokio::net::{TcpStream, TcpListener};
use tokio_codec::*;
use tokio::prelude;

use futures::future::{self, Either, ok, loop_fn, Future, FutureResult, Loop};



fn handle_connecting(state: Connecting) -> Box<Future<Item = Loop<ServerState, ServerState>, Error = io::Error> + Send> {
    let res = state.framed_socket.send("220 smtp.server.com Simple Mail Transfer Service Ready\r".into())
        .and_then(|framed_socket| {
            let new_state = ServerState::Connected(Connected { framed_socket });
            let result = Loop::Continue::<ServerState, ServerState>(new_state);
            Ok(result)
        });

    return Box::new(res);
}


fn handle_connected(state: Connected) -> Box<Future<Item = Loop<ServerState, ServerState>, Error = io::Error> + Send> {
    // C: HELO client.example.com
    // S: 250 Hello client.example.com

    println!("Making future");
    let res = state.framed_socket
        .into_future()
        .and_then(|(line, framed_socket)| {
            println!("Read from socket");
            let msg = line.unwrap();
            println!("Read message {:?}", msg);
            let ss = match msg.as_ref() {
                    "exit"  => ServerState::Disconnected(Disconnected { }),
                    _       => ServerState::Connected(Connected { framed_socket }),
                };

            Ok(Loop::Continue::<ServerState, ServerState>(ss))
        })
        .map_err(|(e, _)| e);

    return Box::new(res);

}

fn disconnect(state: Disconnected) -> impl Future<Item = ServerState, Error = io::Error> {
    result::<ServerState, io::Error>(Ok(ServerState::Disconnected(state)))
}



// pub fn resolve_and_connect(host: &str) -> ResolveAndConnect {
//     let state = State::Resolving(resolve(host));
//     ResolveAndConnect { state }
// }



// fn handle_session_started(state: SessionStarted) -> Result<ServerState, Error> {
//     // C: MAIL FROM:<mail@samlogic.com>
//     // S: 250 OK
    
//     read_client_command(&state.stream)
//         .and_then(|cmd| {
//             match cmd {
//                 ClientCommand::Mail { reverse_path } => {
//                     respond_to_client(SmtpResponse::Okay, &state.stream)
//                         .map(|()| {
//                             ServerState::TransactionStarted(TransactionStarted {
//                                 listener: state.listener,
//                                 stream: state.stream,
//                                 client_socket_addr: state.client_socket_addr,
//                                 envelope: SMTPEnvelope {
//                                     reverse_path: reverse_path,
//                                     forward_paths: Vec::new(),
//                                 },
//                             })
//                         })
//                 },
//                 ClientCommand::Invalid { invalid_cmd } => {
//                     println!("Invalid command: '{}'", invalid_cmd.trim());

//                    respond_to_client(SmtpResponse::CommandUnrecognized, &state.stream)
//                         .map(|()| {
//                             ServerState::SessionStarted(state)
//                         })
//                 },
//                 _ => panic!()
//             }
//         })
// }

// fn handle_transaction_started(state: TransactionStarted) -> Result<ServerState, Error> {
//     // C: RCPT TO:<john@mail.com>
//     // S: 250 OK

//     read_client_command(&state.stream)
//         .and_then(|cmd| {
//             match cmd {
//                 ClientCommand::Recipient { forward_path } => {
//                     let mut updated_forward_paths = state.envelope.forward_paths.clone();
//                     updated_forward_paths.push(forward_path);

//                     respond_to_client(SmtpResponse::Okay, &state.stream)
//                         .map(|()| {
//                             ServerState::TransactionReady(TransactionReady {
//                                 listener: state.listener,
//                                 stream: state.stream,
//                                 client_socket_addr: state.client_socket_addr,
//                                 envelope: SMTPEnvelope {
//                                     reverse_path: state.envelope.reverse_path,
//                                     forward_paths: updated_forward_paths,
//                                 }
//                             })
//                         })
//                 },
//                 ClientCommand::Invalid { invalid_cmd } => {
//                     println!("Invalid command: '{}'", invalid_cmd.trim());

//                    respond_to_client(SmtpResponse::CommandUnrecognized, &state.stream)
//                         .map(|()| {
//                             ServerState::TransactionStarted(state)
//                         })
//                 },
//                 _ => panic!()
//             }
//         })
// }

// fn handle_transation_ready(state: TransactionReady) -> Result<ServerState, Error> {
//     // C: DATA
//     // S: 354 Send message content; end with <CRLF>.<CRLF>
    
//     let abc = read_client_command(&state.stream)
//             .and_then(|cmd| {
//                 match cmd {
//                     ClientCommand::Data => {
//                         println!("Going into DATA mode");

//                         respond_to_client(SmtpResponse::BeginData, &state.stream)
//                             .map(|()| {
//                                 ServerState::DataMode(DataMode {
//                                     listener: state.listener,
//                                     stream: state.stream,
//                                     client_socket_addr: state.client_socket_addr,
//                                     envelope: SMTPEnvelope {
//                                         reverse_path: state.envelope.reverse_path,
//                                         forward_paths: state.envelope.forward_paths,
//                                     }
//                                 })
//                             })
//                     },
//                     ClientCommand::Recipient { forward_path } => {
//                         println!("forward_path: {}", forward_path);

//                         let mut updated_forward_paths = state.envelope.forward_paths.clone();
//                         updated_forward_paths.push(forward_path);

//                         respond_to_client(SmtpResponse::Okay, &state.stream)
//                             .map(|()| {
//                                 ServerState::TransactionReady(TransactionReady {
//                                     listener: state.listener,
//                                     stream: state.stream,
//                                     client_socket_addr: state.client_socket_addr,
//                                     envelope: SMTPEnvelope {
//                                         reverse_path: state.envelope.reverse_path,
//                                         forward_paths: updated_forward_paths,
//                                     }
//                                 })
//                             })
//                     },
//                     ClientCommand::Invalid { invalid_cmd } => {
//                         println!("Invalid command: '{}'", invalid_cmd.trim());

//                     respond_to_client(SmtpResponse::CommandUnrecognized, &state.stream)
//                             .map(|()| {
//                                 ServerState::TransactionReady(state)
//                             })
//                     },
//                     _ => panic!()
//                 }
//             });

//     return abc;
// }

// fn handle_data_mode(state: DataMode) -> Result<ServerState, Error> {
//     // C: <The message data (body text, subject, e-mail header, attachments etc) is sent>
//     // C: .
//     // S: 250 OK, message accepted for delivery: queued as 12345

//     read_client_data(&state.stream)
//         .and_then(|data| {
//             println!("Data recieved, {} bytes.", data.len());
//             println!("{}", data);
//             respond_to_client(SmtpResponse::Okay, &state.stream)
//         })
//         .map(|()| {
//             ServerState::Connected(Connected {
//                 listener: state.listener,
//                 stream: state.stream,
//                 client_socket_addr: state.client_socket_addr
//             })
//         })
// }








pub fn main() {
    let addr = "127.0.0.1:6142".parse().unwrap();

    // Bind a TCP listener to the socket address.
    //
    // Note that this is the Tokio TcpListener, which is fully async.
    let listener = TcpListener::bind(&addr).unwrap();

    // The server task asynchronously iterates over and processes each
    // incoming connection.
    let server = listener.incoming().for_each(move |socket| {

        let framed_socket = Framed::new(socket, LinesCodec::new());

        let initial_state: ServerState = ServerState::Connecting(Connecting{ framed_socket });

        let smtp_process = loop_fn(initial_state, |smtp_session| {
            println!("Loop Fn");

            match smtp_session {
                ServerState::Connecting(smtp_session) => {
                    println!("In Connecting state");
                    return handle_connecting(smtp_session);
                }
                ServerState::Connected(smtp_session) => {
                    println!("In Connected state");
                    return handle_connected(smtp_session);
                }
                _ => {
                    println!("Unknown state, panicing");
                    panic!()
                }
            }
        })
            .and_then(|_res| {
                Ok(())
            })
            .map_err(|_e| ());


        // Spawn a new task that processes the socket:
        tokio::spawn(smtp_process);




        Ok(())
    })
    .map_err(|err| {
        // All tasks must have an `Error` type of `()`. This forces error
        // handling and helps avoid silencing failures.
        //
        // In our example, we are only going to log the error to STDOUT.
        println!("accept error = {:?}", err);
    });

    println!("server running on localhost:6142");

    // Start the Tokio runtime.
    //
    // The Tokio is a pre-configured "out of the box" runtime for building
    // asynchronous applications. It includes both a reactor and a task
    // scheduler. This means applications are multithreaded by default.
    //
    // This function blocks until the runtime reaches an idle state. Idle is
    // defined as all spawned tasks have completed and all I/O resources (TCP
    // sockets in our case) have been dropped.
    //
    // In our example, we have not defined a shutdown strategy, so this will
    // block until `ctrl-c` is pressed at the terminal.
    tokio::run(server);
}


// fn handle_connected(state: Connected) -> Result<ServerState, Error> {
//     // C: HELO client.example.com
//     // S: 250 Hello client.example.com

//     read_client_command(&state.stream)
//         .and_then(|cmd| {
//             match cmd {
//                 ClientCommand::Hello { domain } => {
//                    respond_to_client(SmtpResponse::Okay, &state.stream)
//                         .map(|()| {
//                             ServerState::SessionStarted(SessionStarted {
//                                 listener: state.listener,
//                                 stream: state.stream,
//                                 client_socket_addr: state.client_socket_addr,
//                             })
//                         })
//                 },
//                 ClientCommand::Invalid { invalid_cmd } => {
//                     println!("Invalid command: '{}'", invalid_cmd.trim());

//                    respond_to_client(SmtpResponse::CommandUnrecognized, &state.stream)
//                         .map(|()| {
//                             ServerState::Connected(state)
//                         })
//                 },
//                 _ => panic!()
//             }
//         })
// }

// fn handle_session_started(state: SessionStarted) -> Result<ServerState, Error> {
//     // C: MAIL FROM:<mail@samlogic.com>
//     // S: 250 OK
    
//     read_client_command(&state.stream)
//         .and_then(|cmd| {
//             match cmd {
//                 ClientCommand::Mail { reverse_path } => {
//                     respond_to_client(SmtpResponse::Okay, &state.stream)
//                         .map(|()| {
//                             ServerState::TransactionStarted(TransactionStarted {
//                                 listener: state.listener,
//                                 stream: state.stream,
//                                 client_socket_addr: state.client_socket_addr,
//                                 envelope: SMTPEnvelope {
//                                     reverse_path: reverse_path,
//                                     forward_paths: Vec::new(),
//                                 },
//                             })
//                         })
//                 },
//                 ClientCommand::Invalid { invalid_cmd } => {
//                     println!("Invalid command: '{}'", invalid_cmd.trim());

//                    respond_to_client(SmtpResponse::CommandUnrecognized, &state.stream)
//                         .map(|()| {
//                             ServerState::SessionStarted(state)
//                         })
//                 },
//                 _ => panic!()
//             }
//         })
// }





















//                                                S: 220 smtp.server.com Simple Mail Transfer Service Ready
// C: HELO client.example.com
//                                                S: 250 Hello client.example.com
// C: MAIL FROM:<mail@samlogic.com>
//                                                S: 250 OK
// C: RCPT TO:<john@mail.com>
//                                                S: 250 OK
// C: DATA
//                                                S: 354 Send message content; end with <CRLF>.<CRLF>
// C: <The message data (body text, subject, e-mail header, attachments etc) is sent>
// C: .
//                                                S: 250 OK, message accepted for delivery: queued as 12345
// C: QUIT
//                                                S: 221 Bye

#[macro_use]
extern crate nom;



// 3.1
// MAIL <SP> FROM:<reverse-path> <CRLF>
// RCPT <SP> TO:<forward-path> <CRLF>
// DATA <CRLF>

#[derive(Debug,PartialEq)]
pub struct MailFrom {
    pub reverse_path: String
}

#[derive(Debug,PartialEq)]
pub struct RcptTo {
    pub forward_path: String
}

#[derive(Debug,PartialEq)]
pub struct Data { }


named!(mail_from<&str, MailFrom>,
  do_parse!(
                    tag!("MAIL FROM:")   >>
    reverse_path:   take_until_and_consume!("\r\n") >>
    (MailFrom {
        reverse_path: reverse_path.into()
    })
  )
);


#[derive(Debug,PartialEq)]
pub struct Color {
  pub red:     u8,
  pub green:   u8,
  pub blue:    u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
  u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
  c.is_digit(16)
}

named!(hex_primary<&str, u8>,
  map_res!(take_while_m_n!(2, 2, is_hex_digit), from_hex)
);

named!(hex_color<&str, Color>,
  do_parse!(
           tag!("#")   >>
    red:   hex_primary >>
    green: hex_primary >>
    blue:  hex_primary >>
    (Color { red, green, blue })
  )
);


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

// C: HELO client.example.com

// C: 
    #[test]
    fn test_mail_from_parsing() {
        assert_eq!(mail_from("MAIL FROM:<mail@samlogic.com>\r\n"), Ok(("", MailFrom {
            reverse_path: "<mail@samlogic.com>"
        })));
    }
// C: RCPT TO:<john@mail.com>

// C: DATA

// C: QUIT



    #[test]
    fn test_add() {
        assert_eq!(hex_color("#2F14DF"), Ok(("", Color {
            red: 47,
            green: 20,
            blue: 223,
        })));
    }
}