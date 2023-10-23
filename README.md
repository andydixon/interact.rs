# interact.rs
### An unofficial Rust crate for the Webex Interact API

**Example Usage**

```rust
use Interact::sms;

fn main() {
    let mut sms = sms::sms_api("zzz_XXXXXXXXXXXXXXXXXXXXXXXXXX".to_string());
    match sms
        .add_recipient("+447000000000".to_string()) // Add as many times as you need
        .message("Testing using new Interact Rust class".to_string())
        .set_originator("RustAPI".to_string())
        .send_sms() {
            Ok(response) => println!("Success! {},{}", response.status, response.response_body),
            Err(e) => {
                println!("Error! {}",e);
                match e.get_data() {
                    Some(data) => {
                        println!("Additional information: {}",data);
                    },
                    None => {
                        println!("No additional data available");
                    }
                }
            }
        }
}
```