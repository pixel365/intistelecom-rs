use intistelecom_rs::{
    client::client::Client,
    model::originator::BaseOriginator,
    originator::originator::{all, create, delete, set_default},
};

fn main() {
    let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");

    let res = all(&client);
    println!("{:#?}", res);

    let res = create(
        &client,
        &BaseOriginator {
            originator: String::from("test_originator"),
            default: false,
        },
    );
    println!("{:#?}", res);

    let res = set_default(&client, "test_originator");
    println!("{:#?}", res);

    let res = delete(&client, "test_originator");
    println!("{:#?}", res);
}
