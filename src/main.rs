use std::{collections::HashMap, env};

use cli_ckup::{
    GetFolderlessListsRequest, GetSpaceRequest, GetSpacesRequest, GetTasksRequest, ListsResponse,
    Space, SpacesResponse, TasksResponse, UserResponse, WorkspacesResponse,
};
use reqwest::{Error, Method};

static ENV_NAME_API_KEY: &str = "CLICKUP_API_KEY";

static BASE_URL: &str = "https://api.clickup.com/api/v2";

static HEADER_NAME_AUTHORIZATION: &str = "authorization";

struct Client<'a> {
    client: reqwest::blocking::Client,
    base_url: &'a str,
    api_key: &'a str,
}

impl Client<'_> {
    fn get_user(&self) -> Result<UserResponse, Error> {
        self.client
            .request(Method::GET, format!("{}/user", self.base_url))
            .header(HEADER_NAME_AUTHORIZATION, self.api_key)
            .send()?
            .json()
    }

    fn get_workspaces(&self) -> Result<WorkspacesResponse, Error> {
        self.client
            .request(Method::GET, format!("{}/team", self.base_url))
            .header(HEADER_NAME_AUTHORIZATION, self.api_key)
            .send()?
            .json()
    }

    fn get_spaces(&self, req: GetSpacesRequest) -> Result<SpacesResponse, Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/team/{}/space", self.base_url, req.team_id),
            )
            .query(
                &vec![req.archived.map(|v| ("archived", v.to_string()))]
                    .into_iter()
                    .flatten()
                    .collect::<HashMap<&str, String>>(),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key)
            .send()?
            .json()
    }

    fn get_space(&self, req: GetSpaceRequest) -> Result<Space, Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/space/{}", self.base_url, req.space_id),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key)
            .send()?
            .json()
    }

    fn get_folderless_lists(&self, req: GetFolderlessListsRequest) -> Result<ListsResponse, Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/space/{}/list", self.base_url, req.space_id),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key)
            .send()?
            .json()
    }

    fn get_tasks(&self, req: GetTasksRequest) -> Result<TasksResponse, Error> {
        self.client
            .request(
                Method::GET,
                format!("{}/list/{}/task", self.base_url, req.list_id),
            )
            .header(HEADER_NAME_AUTHORIZATION, self.api_key)
            .send()?
            .json()
    }
}

fn main() -> Result<(), Error> {
    let client = Client {
        client: reqwest::blocking::Client::new(),

        base_url: &BASE_URL.to_string(),

        api_key: &env::var(ENV_NAME_API_KEY)
            .expect(format!("{} environment variable not set", ENV_NAME_API_KEY).as_str()),
    };

    let workspaces = client.get_workspaces()?;

    let spaces = client.get_spaces(GetSpacesRequest {
        team_id: workspaces.teams[0]
            .id
            .parse()
            .expect("team id is not a number"),
        archived: None,
    })?;

    let lists = client.get_folderless_lists(GetFolderlessListsRequest {
        space_id: spaces.spaces[0]
            .id
            .parse()
            .expect("space id is not a number"),
    })?;

    let response = client.get_tasks(GetTasksRequest {
        list_id: lists.lists[0].id.parse().expect("list id is not a number"),
    })?;

    println!("{:#?}", response);

    Ok(())
}
