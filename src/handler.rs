use crate::print::print_by_cfg;
use crate::{cli, client, config};

pub fn user_view(cfg: &config::Config) {
    let value = client::from_config(cfg).get_user().unwrap();

    print_by_cfg(cfg, &vec![value.user]);
}

pub fn workspace_view(cfg: &config::Config) {
    let value = client::from_config(cfg).get_workspaces().unwrap();

    print_by_cfg(cfg, &value.teams);
}

pub fn space_view(cfg: &config::Config, args: &cli::SpaceViewArgs) {
    let value = client::from_config(cfg)
        .get_spaces(crate::model::GetSpacesRequest {
            workspace_id: args.workspace_id,
            archived: None,
        })
        .unwrap();

    print_by_cfg(cfg, &value.spaces);
}

pub fn list_view(cfg: &config::Config, args: &cli::ListViewArgs) {
    let value = client::from_config(cfg)
        .get_folderless_lists(crate::model::GetFolderlessListsRequest {
            space_id: args.space_id,
        })
        .unwrap();

    print_by_cfg(cfg, &value.lists);
}

pub fn task_view(cfg: &config::Config, args: &cli::TaskViewArgs) {
    let value = client::from_config(cfg)
        .get_tasks(crate::model::GetTasksRequest {
            list_id: args.list_id,
        })
        .unwrap();

    print_by_cfg(cfg, &value.tasks);
}
