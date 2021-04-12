extern crate imap;
extern crate native_tls;

use mailparse::parse_headers;
use mailparse::parse_mail;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Hello, world!");

    let domain = "imap.gmail.com";

    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    let mut imap_session = client
        .login("qizengtai@gmail.com", "yrwqwrhlfxtsfapt")
        .expect("Failed to login.");

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX").expect("Failed to open INBOX");

    let header_messages = imap_session
        .fetch("1:100", "BODY[HEADER]")
        .expect("Failed to fetched header messages.");

    let mut index = 1;
    let mut found = false;
    'outer: for message in header_messages.iter() {
        let header = message
            .header()
            .expect("Failed to read a fetched header message.");
        let (parsed_header, _) =
            parse_headers(header).expect("Failed to parse headers of a message.");
        for header in parsed_header {
            if header.get_key().to_lowercase() == "subject"
                && header.get_value().to_lowercase() == "smart meter texas – subscription report"
            {
                found = true;
                break 'outer;
            }
        }
        index += 1;
    }
    if !found {
        println!("Didn't find THE message");
        return;
    }

    let the_message = imap_session
        .fetch(format!("{}", index), "BODY[]")
        .expect("Failed to fetch THE message.");

    let message = the_message
        .iter()
        .next()
        .expect("Something wrong with THE fetched message.");
    let body = message.body().expect("Failed to read body of THE message.");
    let parsed_body = parse_mail(body).expect("Failed to parse THE messsage.");
    let mut found = false;
    for subpart in parsed_body.subparts {
        if subpart
            .ctype
            .mimetype
            .to_lowercase()
            .contains("application/xml")
        {
            found = true;
            let csv_data = subpart
                .get_body_raw()
                .expect("Faild to read THE attachment.");
            let file_name = &subpart.get_content_disposition().params["filename"];
            let mut pos = 0;
            let mut buffer = File::create(format!("/Users/shane/Downloads/{}", file_name))
                .expect("Failed to create the csv file.");
            while pos < csv_data.len() {
                let bytes_written = buffer.write(&csv_data[pos..]).expect("Failed to write a byte.");
                pos += bytes_written;
            }
        }
    }
    if !found {
        panic!("Didn't find THE attachment.")
    }

    // imap_session.mv(format!("{}", index), "Archive").expect("Failed to archive THE message..");

    imap_session.logout().expect("Failed to logout.");
}

// fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
//     let domain = "imap.gmail.com";
//     let tls = native_tls::TlsConnector::builder().build().unwrap();

//     // we pass in the domain twice to check that the server's TLS
//     // certificate is valid for the domain we're connecting to.
//     let client = imap::connect((domain, 993), domain, &tls).unwrap();

//     // the client we have here is unauthenticated.
//     // to do anything useful with the e-mails, we need to log in
//     let mut imap_session = client
//         .login("qizengtai@gmail.com", "yrwqwrhlfxtsfapt")
//         .map_err(|e| e.0)?;

//     // we want to fetch the first email in the INBOX mailbox
//     imap_session.select("INBOX")?;

//     let header_messages = imap_session
//         .fetch("1:100", "BODY[HEADER]")
//         .expect("Failed to fetched header messages.");
//     let mut index = 0;
//     'outer: for message in header_messages.iter() {
//         let header = message
//             .header()
//             .expect("Failed to read a fetched header message.");
//         let (parsed_header, _) =
//             parse_headers(header).expect("Failed to parse headers of a message.");
//         for header in parsed_header {
//             if header.get_key().to_lowercase() == "subject"
//                 && header.get_value().to_lowercase() == "smart meter texas – subscription report"
//             {
//                 break 'outer;
//             }
//         }
//         index += 1;
//     }

//     let the_message = imap_session.fetch("1:100", "BODY[HEADER]")?;

//     {
//         // for one_header in parsed {
//         //     let key = one_header.get_key();
//         //     if key == "Subject" {
//         //         let sub = one_header.get_value();
//         //         if sub == "Smart Meter Texas – Subscription Report" {
//         println!("{:?}", message.bodystructure());
//         let body = message.body().expect("message did not have a body!");
//         // let body = std::str::from_utf8(body)
//         //     .expect("message was not valid utf-8")
//         //     .to_string();
//         // let bs = message.bodystructure();
//         // println!("{:?}", body);
//         use mailparse::parse_mail;
//         let parse = parse_mail(body).unwrap();
//         if parse.subparts.len() > 1 {
//             // println!("{:?}", parse.subparts[1].get_body());
//         }
//         println!("======");
//         println!("======");
//         println!("======");
//         println!("======");
//         //         }
//         //         println!("{}", sub);
//         //     }
//         // }

//         // let uid = message.uid.expect("message did not have a body!");
//         // println!("{}", uid);
//     }

//     // let header = std::str::from_utf8(header)
//     //     .expect("message was not valid utf-8")
//     //     .to_string();

//     // be nice to the server and log out
//     imap_session.logout()?;

//     Ok(Some("subject".to_string()))
// }
