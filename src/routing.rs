#[allow(dead_code, unreachable_code)]

pub mod routing {
    use std::error::Error;

    use crate::{client::client::Requests, model::routing::Routing};

    /// Cost of message
    ///
    /// ```rust,no_run
    /// use intistelecom_rs::{client::client::Client, routing::routing::cost};
    ///  
    /// fn main() {
    ///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    ///     let res = cost(&client, "PHONE_NUMBER");
    ///     println!("{:#?}", res);
    /// }
    /// ```
    pub fn cost(client: &impl Requests, phone_number: &str) -> Result<Routing, Box<dyn Error>> {
        let resp = client.get(&format!("/routing/{}", phone_number))?;
        let routing: Routing = serde_json::from_str(&resp)?;
        Ok(routing)
    }
}
