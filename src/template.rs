#[allow(dead_code, unreachable_code)]

pub mod template {
    use std::error::Error;

    use crate::{
        client::client::Requests,
        model::template::{BaseTemplate, Template},
    };

    /// List of templates
    ///
    /// ```rust,no_run
    /// use intistelecom_rs::{
    ///     client::client::Client,
    ///     template::template::all,
    /// };
    ///  
    /// fn main() {
    ///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    ///     let res = all(&client);
    ///     println!("{:#?}", res);
    /// }
    /// ```
    pub fn all(client: &impl Requests) -> Result<Vec<Template>, Box<dyn Error>> {
        let resp = client.get("/template")?;
        let res: Vec<Template> = serde_json::from_str(&resp)?;
        Ok(res)
    }

    /// Create new template
    ///
    /// ```rust,no_run
    /// use intistelecom_rs::{
    ///     client::client::Client,
    ///     model::template::BaseTemplate,
    ///     template::template::create,
    /// };
    ///  
    /// fn main() {
    ///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    ///     let res = create(
    ///         &client,
    ///         &mut BaseTemplate {
    ///             template: String::from("some template text"),
    ///             title: String::from("some title"),
    ///             id: 0,
    ///         },
    ///     );
    ///     println!("{:#?}", res);
    /// }
    /// ```
    pub fn create(
        client: &impl Requests,
        template: &BaseTemplate,
    ) -> Result<Template, Box<dyn Error>> {
        return self::request(client, template, reqwest::Method::POST);
    }

    /// Edit an existing template
    ///
    /// ```rust,no_run
    /// use intistelecom_rs::{
    ///     client::client::Client,
    ///     model::template::BaseTemplate,
    ///     template::template::edit,
    /// };
    ///  
    /// fn main() {
    ///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    ///     let res = edit(
    ///         &client,
    ///         &mut BaseTemplate {
    ///             template: String::from("some template text"),
    ///             title: String::from("some title"),
    ///             id: 123,
    ///         },
    ///     );
    ///     println!("{:#?}", res);
    /// }
    /// ```
    pub fn edit(
        client: &impl Requests,
        template: &BaseTemplate,
    ) -> Result<Template, Box<dyn Error>> {
        return self::request(client, template, reqwest::Method::PUT);
    }

    /// Delete template
    ///
    /// ```rust,no_run
    /// use intistelecom_rs::{
    ///     client::client::Client,
    ///     template::template::delete,
    /// };
    ///  
    /// fn main() {
    ///     let client: Client = Client::new("YOUR_USERNAME", "YOUR_API_KEY");
    ///     let res = delete(&client, "test_originator");
    ///     println!("{:#?}", res);
    /// }
    pub fn delete(client: &impl Requests, template_id: u32) -> bool {
        let resp = client.delete(&format!("/template/{}", template_id));
        match resp {
            Ok(_) => true,
            Err(_) => false,
        }
    }

    fn request(
        client: &impl Requests,
        template: &BaseTemplate,
        method: reqwest::Method,
    ) -> Result<Template, Box<dyn Error>> {
        let body = serde_json::to_string(template)?;

        let resp = match method {
            reqwest::Method::PUT => client.put("/template", Some(body))?,
            _ => client.post("/template", body)?,
        };
        let res: Template = serde_json::from_str(&resp)?;
        Ok(res)
    }
}
