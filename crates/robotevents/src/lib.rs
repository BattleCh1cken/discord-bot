use reqwest;
use reqwest::header::USER_AGENT;
use std::time::Duration;

mod model;
use model::*;

#[derive(Debug)]
pub struct Client {
    client: reqwest::Client,
    token: String,
}

impl Client {
    pub fn new(token: String) -> Self {
        let reqwest_client = reqwest::Client::new();
        Client {
            client: reqwest_client,
            token,
        }
    }

    async fn make_request(&self, url: String) -> Result<reqwest::Response, reqwest::Error> {
        Ok(self
            .client
            .get(url)
            .header("accept-language", "en")
            .header(USER_AGENT, "Fred bot")
            .bearer_auth(&self.token)
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .unwrap())
    }

    pub async fn get_team(&self, search: &str) -> Result<Vec<Team>, reqwest::Error> {
        let url = format!("https://www.robotevents.com/api/v2/teams?number={}", search);
        let request = self.make_request(url).await?;

        let body = request.json::<RobotEventsResponse<model::Team>>().await?;
        Ok(body.data)
    }

    pub async fn get_skills(
        &self,
        search: &str,
    ) -> Result<Vec<model::skills::Skill>, reqwest::Error> {
        let team = &self.get_team(search).await?[0];
        let url = format!(
            "https://www.robotevents.com/api/v2/teams/{}/skills",
            team.id
        );
        let request = self.make_request(url).await?;

        let body = request
            .json::<RobotEventsResponse<model::skills::Skill>>()
            .await?;

        Ok(body.data)
    }
}
