use crate::config;
use crate::model;
use reqwest::Method;
use std::collections::HashMap;

static BASE_URL: &str = "https://api.clickup.com/api/v2";

static HEADER_NAME_AUTHORIZATION: &str = "authorization";

pub struct Client {
    pub client: reqwest::blocking::Client,
    pub base_url: String,
    pub api_key: String,
}

pub fn from_config(config: &config::Config) -> Client {
    Client {
        client: reqwest::blocking::Client::new(),
        base_url: BASE_URL.to_string(),
        api_key: config.api_key.clone(),
    }
}

impl Client {
    pub fn get_user(&self) -> Result<model::UserResponse, reqwest::Error> {
        self.client
            .request(Method::GET, format!("{}/user", self.base_url))
            .header(HEADER_NAME_AUTHORIZATION, self.api_key.as_str())
            .send()?
            .json()
    }

    pub fn get_workspaces(&self) -> Result<model::WorkspacesResponse, reqwest::Error> {
        self.client
            .request(Method::GET, format!("{}/team", self.base_url))
            .header(HEADER_NAME_AUTHORIZATION, self.api_key.as_str())
            .send()?
            .json()
    }

    pub fn get_spaces(
        &self,
        req: model::GetSpacesRequest,
    ) -> Result<model::SpacesResponse, reqwest::Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/team/{}/space", self.base_url, req.workspace_id),
            )
            .query(
                &vec![req.archived.map(|v| ("archived", v.to_string()))]
                    .into_iter()
                    .flatten()
                    .collect::<HashMap<&str, String>>(),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key.as_str())
            .send()?
            .json()
    }

    pub fn get_space(&self, req: model::GetSpaceRequest) -> Result<model::Space, reqwest::Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/space/{}", self.base_url, req.space_id),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key.as_str())
            .send()?
            .json()
    }

    pub fn get_folderless_lists(
        &self,
        req: model::GetFolderlessListsRequest,
    ) -> Result<model::ListsResponse, reqwest::Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/space/{}/list", self.base_url, req.space_id),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key.as_str())
            .send()?
            .json()
    }

    pub fn get_tasks(
        &self,
        req: model::GetTasksRequest,
    ) -> Result<model::TasksResponse, reqwest::Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/list/{}/task", self.base_url, req.list_id),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key.as_str())
            .send()?
            .json()
    }
}
