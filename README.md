# `mattex`: Nested mail attachment extractor

This is a simple command line tool to extract nested mail attachments from the provided `.eml` file.

### When should I use this tool?

When you forward mails in Outlook, they become `.eml` attachments to a new mail. A recipient using Thunderbird has no possibility of moving the attached mails to his inbox (why not?).

With the [ImportExportTools NG](https://addons.thunderbird.net/thunderbird/addon/importexporttools-ng/) add-on for Thunderbird, the recipient can export the mail to an `.eml` file and use this very tool to extract the attached mails to separate files. They can then be imported again as individual mails into an arbitrary folder using the add-on.

### How should I use this tool?

> As of the newest update, you no longer have to differenciate between received or sent mails. This is automatically detected using the header of the attached mails.

```
Usage:
  mattex <eml input file>       Extract mail attachments
```


An example would be the command
```
mattex messages-attached.eml
```

which, in this example, produces the files
```
- messages-attached-0.eml
- messages-attached-1.eml
- messages-attached-2.eml
- messages-attached-n.eml
- ...
```

### How should I build this tool?

Build mattex using

```
cargo build --release
```