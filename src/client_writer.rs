use std::io::Error;
use tokio::net::TcpStream;
use std::io::prelude::*;

pub enum SmtpResponse {
    // 211 System status, or system help reply
    // 214 Help message
    ServiceReady, // 220 <domain> Service ready
    DisconnectingClient, // 221 <domain> Service closing transmission channel
    Okay, // 250 Requested mail action okay, completed
    RelayOkay, // 251 User not local; will forward to <forward-path>

    BeginData, // 354 Start mail input; end with <CRLF>.<CRLF>

    ServiceUnavailable, // 421 <domain> Service not available,
    // 450 Requested mail action not taken: mailbox unavailable
    // 451 Requested action aborted: local error in processing
    // 452 Requested action not taken: insufficient system storage

    CommandUnrecognized, // 500 Syntax error, command unrecognized
    SyntaxError, // 501 Syntax error in parameters or arguments
    CommandNotImplemented, // 502 Command not implemented
    BadSequence, // 503 Bad sequence of commands
    // 504 Command parameter not implemented
    // 550 Requested action not taken: mailbox unavailable
    // 551 User not local; please try <forward-path>
    // 552 Requested mail action aborted: exceeded storage allocation
    // 553 Requested action not taken: mailbox name not allowed
    TransactionFailed, // 554 Transaction failed
}


pub fn respond_to_client(response: SmtpResponse, mut stream: &TcpStream) -> Result<(), Error> {

    let response_data = match response {
        SmtpResponse::ServiceReady            => "220 Welcome, Service ready\r\n",
        SmtpResponse::DisconnectingClient     => "221 Service closing transmission channel\r\n",
        SmtpResponse::Okay                    => "250 OK\r\n",
        SmtpResponse::RelayOkay               => "251 OK\r\n",

        SmtpResponse::BeginData               => "354 Start mail input; end with <CRLF>.<CRLF>\r\n",

        SmtpResponse::ServiceUnavailable      => "421 Service not available\r\n",

        SmtpResponse::CommandUnrecognized     => "500 Syntax error, command unrecognized\r\n",
        SmtpResponse::SyntaxError             => "501 Syntax error in parameters or arguments\r\n",
        SmtpResponse::CommandNotImplemented   => "502 Command not implemented\r\n",
        SmtpResponse::BadSequence             => "503 Bad sequence of commands\r\n",

        SmtpResponse::TransactionFailed       => "554 Transaction failed\r\n",
    };

    println!("SERVER: {}", response_data.trim());

    stream.write(response_data.as_bytes())
        .and_then(|_bytes_written| {
            stream.flush()
        })
}



// 211 System status, or system help reply
// 214 Help message
// 220 <domain> Service ready
// 221 <domain> Service closing transmission channel
// 250 Requested mail action okay, completed
// 251 User not local; will forward to <forward-path>

// 354 Start mail input; end with <CRLF>.<CRLF>

// 421 <domain> Service not available,
// 450 Requested mail action not taken: mailbox unavailable
// 451 Requested action aborted: local error in processing
// 452 Requested action not taken: insufficient system storage

// 500 Syntax error, command unrecognized
// 501 Syntax error in parameters or arguments
// 502 Command not implemented
// 503 Bad sequence of commands
// 504 Command parameter not implemented
// 550 Requested action not taken: mailbox unavailable
// 551 User not local; please try <forward-path>
// 552 Requested mail action aborted: exceeded storage allocation
// 553 Requested action not taken: mailbox name not allowed
// 554 Transaction failed


    // CONNECTION ESTABLISHMENT
    //    S: 220
    //    F: 421
    // HELO
    //    S: 250
    //    E: 500, 501, 504, 421
    // MAIL
    //    S: 250
    //    F: 552, 451, 452
    //    E: 500, 501, 421
    // RCPT
    //    S: 250, 251
    //    F: 550, 551, 552, 553, 450, 451, 452
    //    E: 500, 501, 503, 421
    // DATA
    //    I: 354 -> data -> S: 250
    //                      F: 552, 554, 451, 452
    //    F: 451, 554
    //    E: 500, 501, 503, 421
    // RSET
    //    S: 250
    //    E: 500, 501, 504, 421
    // SEND
    //    S: 250
    //    F: 552, 451, 452
    //    E: 500, 501, 502, 421
    // SOML
    //    S: 250
    //    F: 552, 451, 452
    //    E: 500, 501, 502, 421
    // SAML
    //    S: 250
    //    F: 552, 451, 452
    //    E: 500, 501, 502, 421
    // VRFY
    //    S: 250, 251
    //    F: 550, 551, 553
    //    E: 500, 501, 502, 504, 421
    // EXPN
    //    S: 250
    //    F: 550
    //    E: 500, 501, 502, 504, 421
    // HELP
    //    S: 211, 214
    //    E: 500, 501, 502, 504, 421
    // NOOP
    //    S: 250
    //    E: 500, 421
    // QUIT
    //    S: 221
    //    E: 500
    // TURN
    //    S: 250
    //    F: 502
    //    E: 500, 503