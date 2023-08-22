use intistelecom_rs::{client::Client, routing::cost};

fn main() {
    let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    let res = cost(&client, "PHONE_NUMBER");
    println!("{:#?}", res);
}
