use mailparse::*;
use regex::Regex;
use std::{env, fs, io, path};

fn help() {
    println!("Usage:");
    println!("  mattex <eml input file>       Extract mail attachments");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 {
        let fp = &args[1];
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
                    process_mails(mails, out_prefix.to_string())
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

fn process_mails(mails: Vec<&str>, out_prefix: String) -> io::Result<()> {
    let re_in = Regex::new(r"Content-Type: message/rfc822(?s).*Received: from").unwrap();
    let re_out = Regex::new(r"Content-Type: message/rfc822(?s).*From: ").unwrap();

    let inbox = mails.iter().filter(|mail| re_in.is_match(mail));
    let outbox = mails.iter().filter(|mail| re_out.is_match(mail));

    let in_c = inbox.clone().count();
    let out_c = outbox.clone().count();

    for (i, mail) in inbox.enumerate() {
        let m = "Received: from".to_string() + &re_in.replace(mail, "").to_string();
        fs::write(format!("{}-{}.eml", out_prefix, i), m)?;
    }
    for (i, mail) in outbox.enumerate() {
        let m = "From: ".to_string() + &re_out.replace(mail, "").to_string();
        fs::write(format!("{}-{}.eml", out_prefix, i), m)?;
    }
    println!("Created {} eml files in total, {} inbox files and {} outbox files.", mails.len(), in_c, out_c);
    Ok(())
}
