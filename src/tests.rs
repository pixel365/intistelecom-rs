#[cfg(test)]
mod tests{
    use crate::{client::{Client, Requests}, message::send, model::message::MessageBody};

    #[test]
    fn test_client(){
        let client = Client::new("username", "api_key");
        assert_eq!(client.basic_authorization(), "Basic dXNlcm5hbWU6YXBpX2tleQ==", "Invalid credentials");
    }

    #[test]
    fn test_send_message_error(){
        let client = Client::new("username", "api_key");
        let resp = send(&client, &mut MessageBody{
            destination: String::from("10009998877"),
            originator: String::from(""),
            text: String::from(""),
            time_to_send: String::from(""),
            validity_period: 0,
            callback_url: String::from("https://my-callback.com"),
            use_local_time: false,
        });
        let err = match resp {
            Ok(_) => false,
            Err(_) => true
        };

        assert_eq!(err, true);
    }
}