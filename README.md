# `mattex`: Nested mail attachment extractor

This is a simple command line tool to extract nested email attachments from some `.eml` file.

### When should I use this tool?

When you forward mails in Outlook, they become `.eml` attachments to a new email. A recipient using Thunderbird has no possibility of moving the attached mails to his inbox (why not?).

With the `ImportExportTools` add-on for Thunderbird, the recipient can export the email to an eml-file and use this very tool to extract the attached mails to separate eml files. Then, they can be imported as proper mails into an arbitrary folder using the add-on again.

### How should I use this tool?

You can either use it in 'Inbox mode', i.e. for received emails, or in 'Sent mode', for emails forwarded from your outbox.

```
mattex -i filename.eml
mattex -o filename.eml
```

### How should I build this tool?

Build mattex with 

```
cargo build
```
