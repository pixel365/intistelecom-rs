use std::error::Error;

use crate::{
    client::Requests,
    model::originator::{BaseOriginator, Originator},
};

/// List of originators
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     originator::all,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = all(&client);
///     println!("{:#?}", res);
/// }
/// ```
pub fn all(client: &impl Requests) -> Result<Vec<Originator>, Box<dyn Error>> {
    let resp = client.get("/originator")?;
    let res: Vec<Originator> = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Create new originator
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     model::originator::BaseOriginator,
///     originator::create,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = create(
///         &client,
///         &mut BaseOriginator {
///             originator: String::from("test_originator"),
///             default: false,
///         },
///     );
///     println!("{:#?}", res);
/// }
/// ```
pub fn create(
    client: &impl Requests,
    originator: &BaseOriginator,
) -> Result<Originator, Box<dyn Error>> {
    let body = serde_json::to_string(originator)?;
    let resp = client.post("/originator", body)?;
    let res: Originator = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Set default originator
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     originator::set_default,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = set_default(&client, "test_originator");
///     println!("{:#?}", res);
/// }
/// ```
pub fn set_default(client: &impl Requests, originator: &str) -> Result<Originator, Box<dyn Error>> {
    let resp = client.put(&format!("/originator/{}", originator), None)?;
    let res: Originator = serde_json::from_str(&resp)?;
    Ok(res)
}

/// Delete originator
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     originator::delete,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = delete(&client, "test_originator");
///     println!("{:#?}", res);
/// }
/// ```
pub fn delete(client: &impl Requests, originator: &str) -> bool {
    let resp = client.delete(&format!("/originator/{}", originator));
    match resp {
        Ok(_) => true,
        Err(_) => false,
    }
}
