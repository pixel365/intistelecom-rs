use std::error::Error;

use crate::{
    client::Requests,
    model::user::{Balance, User},
};

/// Account balance
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     user::balance,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = balance(&client);
///     println!("{:#?}", res);
/// }
/// ```
pub fn balance(client: &impl Requests) -> Result<Balance, Box<dyn Error>> {
    let resp = client.get("/user/balance")?;
    let balance: Balance = serde_json::from_str(&resp)?;
    Ok(balance)
}

/// Account details
///
/// ```rust,no_run
/// use intistelecom_rs::{
///     client::Client,
///     user::me,
/// };
///  
/// fn main() {
///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
///     let res = me(&client);
///     println!("{:#?}", res);
/// }
pub fn me(client: &impl Requests) -> Result<User, Box<dyn Error>> {
    let resp = client.get("/user/me")?;
    let user: User = serde_json::from_str(&resp)?;
    Ok(user)
}
