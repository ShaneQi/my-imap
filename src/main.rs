extern crate dotenv;
extern crate imap;
extern crate native_tls;

use dotenv::dotenv;
use mailparse::parse_headers;
use mailparse::parse_mail;
use simple_logger::SimpleLogger;
use std::fs::File;
use std::io::prelude::*;
use std::thread;
use std::time::Duration;
use std::vec;

fn main() {
    dotenv().ok();
    let interval = dotenv::var("INTERVAL").unwrap().parse::<u64>().unwrap();
    SimpleLogger::new().init().unwrap();
    log::info!("START");
    loop {
        let electricity_meter_result = electricity_meter();
        match electricity_meter_result {
            Ok(_) => log::info!("Successful Electricity Meter task."),
            Err(error) => log::error!("Failed Electricity Meter task, error: {:?}.", error),
        }
        log::info!("Sleeping for {} seconds.", interval);
        thread::sleep(Duration::from_secs(interval));
    }
}

#[derive(Debug)]
enum ElectricityMeterError {
    EnvVar,
    TLS,
    Login,
    SelectInbox,
    FetchHeaders,
    ReadHeaders,
    ParseHeaders,
    FetchBodies,
    ReadBodies,
    ParseBodies,
    ReadAttachment,
    FindAttachment,
    CreateAttachment(String),
    WriteAttachment(String),
    Archive,
    Logout,
}

fn electricity_meter() -> std::result::Result<(), ElectricityMeterError> {
    let domain = dotenv::var("IMAP_SERVER").map_err(|_| ElectricityMeterError::EnvVar)?;
    let email = dotenv::var("EMAIL").map_err(|_| ElectricityMeterError::EnvVar)?;
    let password = dotenv::var("PASSWORD").map_err(|_| ElectricityMeterError::EnvVar)?;
    // Don't include the ending `/`.
    let electricity_meter_file_path =
        dotenv::var("ELECTRICITY_METER_FILE_PATH").map_err(|_| ElectricityMeterError::EnvVar)?;
    let tls = native_tls::TlsConnector::builder()
        .build()
        .map_err(|_| ElectricityMeterError::TLS)?;
    let client = imap::connect((domain.clone(), 993), domain.clone(), &tls)
        .map_err(|_| ElectricityMeterError::TLS)?;

    log::info!("Establishing connection.");
    let mut imap_session = client
        .login(email, password)
        .map_err(|_| ElectricityMeterError::Login)?;

    log::info!("Selecting INBOX.");
    imap_session
        .select("INBOX")
        .map_err(|_| ElectricityMeterError::SelectInbox)?;

    log::info!("Fetching headers.");
    let header_messages = imap_session
        .fetch("1:100", "BODY[HEADER]")
        .map_err(|_| ElectricityMeterError::FetchHeaders)?;

    let mut found_sequences: vec::Vec<u8> = vec![];
    let mut index = 1;
    for message in header_messages.iter() {
        let header = message.header().ok_or(ElectricityMeterError::ReadHeaders)?;
        let (parsed_header, _) =
            parse_headers(header).map_err(|_| ElectricityMeterError::ParseHeaders)?;
        for header in parsed_header {
            if header.get_key().to_lowercase() == "subject" {
                let subject = header.get_value();
                if subject.to_lowercase() == "smart meter texas – subscription report" {
                    log::info!("Found message with subject: {}.", subject);
                    found_sequences.push(index);
                } else {
                    log::info!("Skipping message with subject: {}.", subject);
                }
            }
        }
        index += 1;
    }
    if found_sequences.len() == 0 {
        log::warn!("Didn't find any Electricity Meter messages.");
        return Ok(());
    }

    let query_sequences_strings: vec::Vec<String> = found_sequences
        .iter()
        .map(|x| -> String { return format!("{}", x) })
        .collect();
    let query_sequences = query_sequences_strings.join(",");
    log::info!("Query sequences for fetching bodies: {}.", query_sequences);

    log::info!("Fetching bodies");
    let electricity_meter_messages = imap_session
        .fetch(query_sequences.clone(), "BODY[]")
        .map_err(|_| ElectricityMeterError::FetchBodies)?;
    for message in electricity_meter_messages.iter() {
        let body = message.body().ok_or(ElectricityMeterError::ReadBodies)?;
        let parsed_body = parse_mail(body).map_err(|_| ElectricityMeterError::ParseBodies)?;
        log::info!("Subparts count: {}.", parsed_body.subparts.len());
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
                    .map_err(|_| ElectricityMeterError::ReadAttachment)?;
                let file_name = &subpart.get_content_disposition().params["filename"];
                log::info!("Found application/xml subpart, filename: {}.", file_name);
                log::info!("Writing.");
                let mut pos = 0;
                let mut buffer =
                    File::create(format!("{}/{}", electricity_meter_file_path, file_name))
                        .map_err(|_| {
                            ElectricityMeterError::CreateAttachment(file_name.to_string())
                        })?;
                while pos < csv_data.len() {
                    let bytes_written = buffer.write(&csv_data[pos..]).map_err(|_| {
                        ElectricityMeterError::WriteAttachment(file_name.to_string())
                    })?;
                    pos += bytes_written;
                }
                log::info!("Writing finished.");
            }
        }
        if !found {
            return Err(ElectricityMeterError::FindAttachment);
        }
    }
    log::info!("Archiving the Electricity Meter messages.");
    imap_session
        .mv(query_sequences.clone(), "Archive")
        .map_err(|_| ElectricityMeterError::Archive)?;

    log::info!("Logging out IMAP client.");
    imap_session
        .logout()
        .map_err(|_| ElectricityMeterError::Logout)?;
    return Ok(());
}
