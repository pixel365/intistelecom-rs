use intistelecom_rs::{
    client::client::Client,
    message::message::{batch, cancel, send, status, status_of_part},
    model::message::{MessageBody, MessageId, PartId},
};

fn main() {
    let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    let res = send(
        &client,
        &mut MessageBody {
            destination: String::from(""),
            originator: String::from("test"),
            text: String::from("test"),
            time_to_send: String::from(""),
            validity_period: 0,
            callback_url: String::from("value"),
            use_local_time: false,
        },
    );
    println!("{:#?}", res);

    let m1 = MessageBody {
        destination: String::from(""),
        originator: String::from("test"),
        text: String::from("test"),
        time_to_send: String::from(""),
        validity_period: 0,
        callback_url: String::from("value"),
        use_local_time: false,
    };
    let m2 = MessageBody {
        destination: String::from(""),
        originator: String::from("test"),
        text: String::from("test"),
        time_to_send: String::from(""),
        validity_period: 0,
        callback_url: String::from("value"),
        use_local_time: false,
    };

    let res = batch(&client, &mut vec![m1, m2]);
    println!("{:?}", res);

    let res = status(
        &client,
        &MessageId {
            id: "MESSAGE_ID".to_string(),
        },
    );
    println!("{:?}", res);

    let res = cancel(
        &client,
        &MessageId {
            id: "MESSAGE_ID".to_string(),
        },
    );
    println!("{:?}", res);

    let res = status_of_part(
        &client,
        &PartId {
            id: "PART_ID".to_string(),
        },
    );
    println!("{:?}", res);
}
