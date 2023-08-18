use intistelecom_rs::{
    client::client::Client,
    model::template::BaseTemplate,
    template::template::{all, create, delete, edit},
};

fn main() {
    let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");

    let res = all(&client);
    println!("{:#?}", res);

    let res = create(
        &client,
        &BaseTemplate {
            template: String::from("some template text"),
            title: String::from("some title"),
            id: 0,
        },
    );
    println!("{:#?}", res);

    let res = edit(
        &client,
        &BaseTemplate {
            template: String::from("some template text"),
            title: String::from("some title"),
            id: 1,
        },
    );
    println!("{:#?}", res);

    let res = delete(&client, 123);
    println!("{:#?}", res);
}
