extern crate imap;
extern crate native_tls;
extern crate dotenv;

use mailparse::parse_headers;
use mailparse::parse_mail;
use std::fs::File;
use std::io::prelude::*;
use std::vec;
use dotenv::dotenv;
  
fn main() {
    dotenv().ok();
    let domain = dotenv::var("IMAP_SERVER").unwrap();
    let email = dotenv::var("EMAIL").unwrap();
    let password = dotenv::var("PASSWORD").unwrap();
      
    let tls = native_tls::TlsConnector::builder().build().unwrap();
    let client = imap::connect((domain.clone(), 993), domain.clone(), &tls).unwrap();

    let mut imap_session = client
        .login(email, password)
        .expect("Failed to login.");

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX").expect("Failed to open INBOX");

    let header_messages = imap_session
        .fetch("1:100", "BODY[HEADER]")
        .expect("Failed to fetched header messages.");

    let mut found_sequences: vec::Vec<u8> = vec![];
    let mut index = 1;
    for message in header_messages.iter() {
        let header = message
            .header()
            .expect("Failed to read a fetched header message.");
        let (parsed_header, _) =
            parse_headers(header).expect("Failed to parse headers of a message.");
        for header in parsed_header {
            if header.get_key().to_lowercase() == "subject"
                && header.get_value().to_lowercase() == "smart meter texas – subscription report"
            {
                found_sequences.push(index);
            }
        }
        index += 1;
    }
    if found_sequences.len() == 0 {
        println!("Didn't find Electricity Meter messages.");
        return;
    }

    let query_sequences_strings: vec::Vec<String> = found_sequences
        .iter()
        .map(|x| -> String { return format!("{}", x) })
        .collect();
    let query_sequences = query_sequences_strings.join(",");

    let electricity_meter_messages = imap_session
        .fetch(query_sequences.clone(), "BODY[]")
        .expect("Failed to fetch Electricity Meter messages.");
    for message in electricity_meter_messages.iter() {
        let body = message
            .body()
            .expect("Failed to read body of the Electricity Meter message.");
        let parsed_body =
            parse_mail(body).expect("Failed to parse the Electricity Meter messsage.");
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
                    .expect("Faild to read the attachment in the Electricity Meter message.");
                let file_name = &subpart.get_content_disposition().params["filename"];
                let mut pos = 0;
                let mut buffer = File::create(format!("/Users/shane/Downloads/{}", file_name))
                    .expect(&format!("Failed to create the csv file {}.", file_name));
                while pos < csv_data.len() {
                    let bytes_written = buffer
                        .write(&csv_data[pos..])
                        .expect("Failed to write a byte.");
                    pos += bytes_written;
                }
            }
        }
        if !found {
            panic!("Failed to find the attachment in the Electricity Meter message.")
        }
    }
    imap_session
        .mv(query_sequences.clone(), "Archive")
        .expect("Failed to archive THE message..");

    imap_session.logout().expect("Failed to logout.");
}
