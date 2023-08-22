# Intistele.com

###### With the intistele.com, you can access and organise SMS mailouts from your own software via API

###### [Intistele.com documentation](https://api-go2.intistele.com/docs/)

### Quickstart

```bash
use intistelecom_rs::{
    client::Client,
    message::send,
    model::message::MessageBody,
};

fn main() {
    let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    let res = send(
        &client,
        &mut MessageBody {
            destination: String::from("PHONE_NUMBER"),
            originator: String::from("ORIGINATOR_NAME"),
            text: String::from("SOME TEXT"),
            time_to_send: String::from("2023-08-01 11:20:00"),
            validity_period: 0,
            callback_url: String::from(""),
            use_local_time: false,
        },
    );
    println!("{:#?}", res);
}

```
