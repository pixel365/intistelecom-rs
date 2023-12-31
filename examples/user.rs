use intistelecom_rs::{
    client::Client,
    user::{balance, me},
};

fn main() {
    let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");

    let res = me(&client);
    println!("{:#?}", res);

    let res = balance(&client);
    println!("{:#?}", res);
}
