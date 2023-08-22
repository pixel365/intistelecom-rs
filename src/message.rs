use std::error::Error;

use chrono::Local;

use crate::{
    client::Requests,
    model::message::{MessageBody, MessageDetails, MessageId, PartId},
};

/// Send message
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     message::send,
///     model::message::MessageBody,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = send(
///         &client,
///         &mut MessageBody {
///             destination: String::from("PHONE_NUMBER"),
///             originator: String::from("ORIGINATOR_NAME"),
///             text: String::from("SOME TEXT"),
///             time_to_send: String::from("2023-08-01 11:20:00"),
///             validity_period: 0,
///             callback_url: String::from(""),
///             use_local_time: false,
///         },
///     );
///     println!("{:#?}", res);
/// }
/// ```
pub fn send(
    client: &impl Requests,
    message: &mut MessageBody,
) -> Result<MessageId, Box<dyn Error>> {
    if message.time_to_send.len() < 1 {
        let now = Local::now();
        message.time_to_send = now.format("%Y-%m-%d %H:%M:%S").to_string();
    }
    let body = serde_json::to_string(message)?;
    let resp = client.post("/message/send", body)?;
    let details: MessageId = serde_json::from_str(&resp)?;
    Ok(details)
}

/// Send multiple messages
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     message::batch,
///     model::message::MessageBody,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let m1 = MessageBody {
///         destination: String::from("+79178880143"),
///         originator: String::from("test"),
///         text: String::from("test"),
///         time_to_send: String::from(""),
///         validity_period: 0,
///         callback_url: String::from("value"),
///         use_local_time: false,
///     };
///     let m2 = MessageBody {
///         destination: String::from("+79178880143"),
///         originator: String::from("test"),
///         text: String::from("test"),
///         time_to_send: String::from(""),
///         validity_period: 0,
///         callback_url: String::from("value"),
///         use_local_time: false,
///     };
///     
///     let res = batch(&client, &mut vec![m1, m2]);
///     println!("{:#?}", res);
/// }
/// ```
pub fn batch(
    client: &impl Requests,
    messages: &mut Vec<MessageBody>,
) -> Result<Vec<MessageId>, Box<dyn Error>> {
    for message in &mut *messages {
        if message.time_to_send.len() < 1 {
            let now = Local::now();
            message.time_to_send = now.format("%Y-%m-%d %H:%M:%S").to_string();
        }
    }

    let body = serde_json::to_string(messages)?;
    let resp = client.post("/message/batch", body)?;
    let details: Vec<MessageId> = serde_json::from_str(&resp)?;
    Ok(details)
}

/// Cancel message
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     message::cancel,
///     model::message::MessageId,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = cancel(&client, &MessageId{id: "MESSAGE_ID".to_string()});
///     println!("{:#?}", res);
/// }
/// ```
pub fn cancel(client: &impl Requests, message_id: &MessageId) -> bool {
    let resp = client.delete(&format!("/message/cancel/{}", message_id.id));
    match resp {
        Ok(_) => true,
        Err(_) => false,
    }
}

/// Status of message
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     message::status,
///     model::message::MessageId,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = status(&client, &MessageId{id: "MESSAGE_ID".to_string()});
///     println!("{:#?}", res);
/// }
/// ```
pub fn status(
    client: &impl Requests,
    message_id: &MessageId,
) -> Result<Vec<MessageDetails>, Box<dyn Error>> {
    let resp = client.get(&format!("/message/status/{}", message_id.id))?;
    let details: MessageDetails = serde_json::from_str(&resp)?;
    Ok(vec![details])
}

/// Status of message part
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     message::status_of_part,
///     model::message::PartId,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = status_of_part(&client, &PartId{id: "MESSAGE_ID".to_string()});
///     println!("{:#?}", res);
/// }
/// ```
pub fn status_of_part(
    client: &impl Requests,
    part_id: &PartId,
) -> Result<Vec<MessageDetails>, Box<dyn Error>> {
    let resp = client.get(&format!("/message/status/part/{}", part_id.id))?;
    let details: MessageDetails = serde_json::from_str(&resp)?;
    Ok(vec![details])
}
