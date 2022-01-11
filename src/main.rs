use mailparse::*;
use regex::Regex;
use core::str::Split;
use std::{fs, env, path};

fn help() {
    println!("Usage:");
    println!(" mattex -i <inbox file>       Extract inbox mail attachments");
    println!(" mattex -o <outbox file>      Extract outbox mail attachments");
}

fn main() {

    let args: Vec<String> = env::args().collect();

    if args.len() > 2 {
        // subcommand: inbox or outbox
        let cmd = &args[1];
        let fp = &args[2];

        let path = path::Path::new(fp);
        
        if path.is_file() {
            let input = fs::read_to_string(fp).expect("input file could not be read!");

            // parse surrounding mail with attachments
            let nested = parse_mail(input.as_bytes()).unwrap();
            let boundary = get_mime_boundary(nested);
            let mails = input.split(&boundary);

            // get file name
            if let Some(out_prefix) = path.file_stem().map(|oss| oss.to_str()).flatten() {
                match &cmd[..] {
                    "-i" => process_inbox(mails, out_prefix.to_string()),
                    "-o" => process_outbox(mails, out_prefix.to_string()),
                    _ => help()
                }
            } else {
                println!("file name empty!");
            }
        } else {
            panic!("file argument is not a file!");
        }
    }
    else {
        help();
    }
}

fn get_mime_boundary(parsed: ParsedMail) -> String {
    // parse headers of surrounding mail with mail attachments
    if let Some(boundary) = parsed
        .ctype.params
        .get("boundary") { 

        println!("found MIME boundary: {}", &boundary);
        boundary.to_string()
    }
    else {
        panic!("File does not contain MIME boundary, aborting.");
    }
}

fn process_inbox(mails: Split<&String>, out_prefix: String) {
    println!("processing inbox ...");
    
    let re = Regex::new(r"Content-Type: message/rfc822(?s).*Received: from").unwrap();
    let mut f_count: u32 = 0;

    for (i, mail) in mails.enumerate() {

        let m_str = mail.trim();
        if !m_str.starts_with("Content-Type: message/rfc822") {
            continue;
        }

        let m = "Received: from".to_string() + &re.replace(m_str, "").to_string();

        let result = fs::write(format!("{}-{}.eml", out_prefix, i), m);
        match result {
            Ok(_) => { f_count += 1 },
            Err(e) => { panic!("Could not write to file: {:?}", e) }
        }
    }
    println!("Created {} eml files.", f_count);
}

fn process_outbox(mails: Split<&String>, out_prefix: String) {
    println!("processing outbox ...");
    
    let re = Regex::new(r"Content-Type: message/rfc822(?s).*From: ").unwrap();
    let mut f_count: u32 = 0;

    for (i, mail) in mails.enumerate() {

        let m_str = mail.trim();
        if !m_str.starts_with("Content-Type: message/rfc822") {
            continue;
        }

        let m = "From: ".to_string() + &re.replace(m_str, "").to_string();

        let result = fs::write(format!("{}-{}.eml", out_prefix, i), m);
        match result {
            Ok(_) => { f_count += 1 },
            Err(e) => { panic!("Could not write to file: {:?}", e) }
        }
    }
    println!("Created {} eml files.", f_count);
}
