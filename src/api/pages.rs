use serde::Deserialize;

pub struct PagesApi {
    base_url: String,
    http_client: reqwest::Client,
}

#[derive(Deserialize, Debug)]
pub struct Page {
    pub id: String,
    pub status: String,
    pub title: String,
    pub body: Body,
}

#[derive(Deserialize, Debug)]
pub struct Body {
    pub storage: Option<Storage>,
}

#[derive(Deserialize, Debug)]
pub struct Storage {
    pub representation: String,
    pub value: String,
}

impl PagesApi {
    pub fn new(base_url: String, http_client: reqwest::Client) -> Self {
        Self {
            base_url,
            http_client,
        }
    }

    pub async fn get_by_id(
        &self,
        id: &str,
        query_params: Option<&str>,
    ) -> Result<Page, reqwest::Error> {
        let mut url = format!("{}/wiki/api/v2/pages/{}", self.base_url, id);
        if let Some(params) = query_params {
            let separator = if params.starts_with('?') { "" } else { "?" };
            url = format!("{}{}{}", url, separator, params);
        }

        let response = self
            .http_client
            .get(&url)
            .send()
            .await?
            .json::<Page>()
            .await;

        return response;
    }
}
