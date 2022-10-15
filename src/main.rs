mod client;
mod model;
use reqwest::Error;
use std::env;

static ENV_NAME_API_KEY: &str = "CLICKUP_API_KEY";

fn main() -> Result<(), Error> {
    let client = client::Client {
        client: reqwest::blocking::Client::new(),
        base_url: &client::BASE_URL.to_string(),
        api_key: &env::var(ENV_NAME_API_KEY)
            .expect(format!("{} environment variable not set", ENV_NAME_API_KEY).as_str()),
    };

    let workspaces = client.get_workspaces()?;

    let spaces = client.get_spaces(model::GetSpacesRequest {
        team_id: workspaces.teams[0]
            .id
            .parse()
            .expect("team id is not a number"),
        archived: None,
    })?;

    let lists = client.get_folderless_lists(model::GetFolderlessListsRequest {
        space_id: spaces.spaces[0]
            .id
            .parse()
            .expect("space id is not a number"),
    })?;

    let response = client.get_tasks(model::GetTasksRequest {
        list_id: lists.lists[0].id.parse().expect("list id is not a number"),
    })?;

    println!("{:#?}", response);

    Ok(())
}
