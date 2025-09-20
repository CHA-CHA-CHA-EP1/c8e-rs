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
}

impl PagesApi {
    pub fn new(base_url: String, http_client: reqwest::Client) -> Self {
        Self {
            base_url,
            http_client,
        }
    }

    pub async fn get_by_id(&self, id: &str) -> Result<Page, reqwest::Error> {
        let url = format!("{}/wiki/api/v2/pages/{}", self.base_url, id);
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
