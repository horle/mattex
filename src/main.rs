use mailparse::*;
use regex::Regex;
use std::{env, fs, io, path};

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
            if let Ok(nested) = parse_mail(input.as_bytes()) {
                let boundary = get_mime_boundary(nested);

                let mails = input
                    .split(&boundary)
                    .filter(|m| m.starts_with("Content-Type: message/rfc822"))
                    .map(|m| m.trim())
                    .collect::<Vec<_>>();

                // get file name
                if let Some(out_prefix) = path.file_stem().map(|oss| oss.to_str()).flatten() {
                    match &cmd[..] {
                        "-i" => process_inbox(mails, out_prefix.to_string()),
                        "-o" => process_outbox(mails, out_prefix.to_string()),
                        _ => {
                            help();
                            Ok(())
                        }
                    }
                    .expect("Error processing mails!");
                } else {
                    panic!("file name empty!");
                }
            } else {
                panic!("Error parsing mail");
            }
        } else {
            panic!("file argument is not a file!");
        }
    } else {
        help();
    }
}

fn get_mime_boundary(parsed: ParsedMail) -> String {
    // parse headers of surrounding mail with mail attachments
    if let Some(boundary) = parsed.ctype.params.get("boundary") {
        println!("found MIME boundary: {}", &boundary);
        boundary.to_string()
    } else {
        panic!("File does not contain MIME boundary, aborting.");
    }
}

fn process_inbox(mails: Vec<&str>, out_prefix: String) -> io::Result<()> {
    println!("processing inbox ...");

    let re = Regex::new(r"Content-Type: message/rfc822(?s).*Received: from").unwrap();

    for (i, mail) in mails.iter().enumerate() {
        let m = "Received: from".to_string() + &re.replace(mail, "").to_string();
        fs::write(format!("{}-{}.eml", out_prefix, i), m)?;
    }
    println!("Created {} eml files.", mails.len());
    Ok(())
}

fn process_outbox(mails: Vec<&str>, out_prefix: String) -> io::Result<()> {
    println!("processing outbox ...");

    let re = Regex::new(r"Content-Type: message/rfc822(?s).*From: ").unwrap();

    for (i, mail) in mails.iter().enumerate() {
        let m = "From: ".to_string() + &re.replace(mail, "").to_string();
        fs::write(format!("{}-{}.eml", out_prefix, i), m)?;
    }
    println!("Created {} eml files.", mails.len());
    Ok(())
}
