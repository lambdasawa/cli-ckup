use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct UserResponse {
    #[serde(rename = "user")]
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "color")]
    pub color: String,

    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<serde_json::Value>,

    #[serde(rename = "initials")]
    pub initials: String,

    #[serde(rename = "week_start_day")]
    pub week_start_day: Option<serde_json::Value>,

    #[serde(rename = "global_font_support")]
    pub global_font_support: Option<bool>,

    #[serde(rename = "timezone")]
    pub timezone: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct WorkspacesResponse {
    #[serde(rename = "teams")]
    pub teams: Vec<Workspace>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Workspace {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "color")]
    pub color: String,

    #[serde(rename = "avatar")]
    pub avatar: Option<serde_json::Value>,

    #[serde(rename = "members")]
    pub members: Vec<Member>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Member {
    #[serde(rename = "user")]
    pub user: User,
}

#[derive(Debug)]
pub struct GetSpacesRequest {
    pub workspace_id: u32,

    pub archived: Option<bool>,
}

#[derive(Debug)]
pub struct GetSpaceRequest {
    pub space_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpacesResponse {
    #[serde(rename = "spaces")]
    pub spaces: Vec<Space>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Space {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "color")]
    pub color: Option<serde_json::Value>,

    #[serde(rename = "private")]
    pub private: bool,

    #[serde(rename = "avatar")]
    pub avatar: Option<serde_json::Value>,

    #[serde(rename = "admin_can_manage")]
    pub admin_can_manage: Option<bool>,

    #[serde(rename = "statuses")]
    pub statuses: Vec<Status>,

    #[serde(rename = "multiple_assignees")]
    pub multiple_assignees: bool,

    #[serde(rename = "features")]
    pub features: Features,

    #[serde(rename = "archived")]
    pub archived: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Features {
    #[serde(rename = "due_dates")]
    pub due_dates: DueDates,

    #[serde(rename = "sprints")]
    pub sprints: CustomFields,

    #[serde(rename = "time_tracking")]
    pub time_tracking: TimeTracking,

    #[serde(rename = "points")]
    pub points: CustomFields,

    #[serde(rename = "custom_items")]
    pub custom_items: CustomFields,

    #[serde(rename = "priorities")]
    pub priorities: Priorities,

    #[serde(rename = "tags")]
    pub tags: CustomFields,

    #[serde(rename = "check_unresolved")]
    pub check_unresolved: CheckUnresolved,

    #[serde(rename = "zoom")]
    pub zoom: CustomFields,

    #[serde(rename = "milestones")]
    pub milestones: CustomFields,

    #[serde(rename = "custom_fields")]
    pub custom_fields: CustomFields,

    #[serde(rename = "dependency_warning")]
    pub dependency_warning: Option<CustomFields>,

    #[serde(rename = "multiple_assignees")]
    pub multiple_assignees: Option<CustomFields>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CheckUnresolved {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "subtasks")]
    pub subtasks: Option<bool>,

    #[serde(rename = "checklists")]
    pub checklists: Option<serde_json::Value>,

    #[serde(rename = "comments")]
    pub comments: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CustomFields {
    #[serde(rename = "enabled")]
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DueDates {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "start_date")]
    pub start_date: bool,

    #[serde(rename = "remap_due_dates")]
    pub remap_due_dates: bool,

    #[serde(rename = "remap_closed_due_date")]
    pub remap_closed_due_date: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priorities {
    #[serde(rename = "enabled")]
    pub enabled: bool,

    #[serde(rename = "priorities")]
    pub priorities: Vec<Priority>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Priority {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "priority")]
    pub priority: String,

    #[serde(rename = "color")]
    pub color: String,

    #[serde(rename = "orderindex")]
    pub order_index: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TimeTracking {
    #[serde(rename = "enabled")]
    enabled: bool,

    #[serde(rename = "harvest")]
    harvest: bool,

    #[serde(rename = "rollup")]
    rollup: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    #[serde(rename = "id")]
    pub id: Option<String>,

    #[serde(rename = "status")]
    pub status: String,

    #[serde(rename = "type")]
    pub status_type: Type,

    #[serde(rename = "orderindex")]
    pub order_index: i64,

    #[serde(rename = "color")]
    pub color: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "closed")]
    Closed,

    #[serde(rename = "custom")]
    Custom,

    #[serde(rename = "open")]
    Open,
}

pub struct GetFolderlessListsRequest {
    pub space_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListsResponse {
    #[serde(rename = "lists")]
    pub lists: Vec<List>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct List {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "orderindex")]
    pub orderindex: i64,

    #[serde(rename = "status")]
    pub status: Option<serde_json::Value>,

    #[serde(rename = "priority")]
    pub priority: Option<serde_json::Value>,

    #[serde(rename = "assignee")]
    pub assignee: Option<serde_json::Value>,

    #[serde(rename = "task_count")]
    pub task_count: i64,

    #[serde(rename = "due_date")]
    pub due_date: Option<serde_json::Value>,

    #[serde(rename = "start_date")]
    pub start_date: Option<serde_json::Value>,

    #[serde(rename = "folder")]
    pub folder: Folder,

    #[serde(rename = "space")]
    pub space: Folder,

    #[serde(rename = "archived")]
    pub archived: bool,

    #[serde(rename = "override_statuses")]
    pub override_statuses: bool,

    #[serde(rename = "permission_level")]
    pub permission_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Folder {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "hidden")]
    pub hidden: Option<bool>,

    #[serde(rename = "access")]
    pub access: bool,
}

pub struct GetTasksRequest {
    pub list_id: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TasksResponse {
    #[serde(rename = "tasks")]
    pub tasks: Vec<Task>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "custom_id")]
    pub custom_id: Option<serde_json::Value>,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "text_content")]
    pub text_content: Option<serde_json::Value>,

    #[serde(rename = "description")]
    pub description: Option<serde_json::Value>,

    #[serde(rename = "status")]
    pub status: Status,

    #[serde(rename = "orderindex")]
    pub orderindex: String,

    #[serde(rename = "date_created")]
    pub date_created: String,

    #[serde(rename = "date_updated")]
    pub date_updated: String,

    #[serde(rename = "date_closed")]
    pub date_closed: Option<String>,

    #[serde(rename = "archived")]
    pub archived: bool,

    #[serde(rename = "creator")]
    pub creator: Creator,

    #[serde(rename = "assignees")]
    pub assignees: Vec<Creator>,

    #[serde(rename = "watchers")]
    pub watchers: Vec<Option<serde_json::Value>>,

    #[serde(rename = "checklists")]
    pub checklists: Vec<Option<serde_json::Value>>,

    #[serde(rename = "tags")]
    pub tags: Vec<Option<serde_json::Value>>,

    #[serde(rename = "parent")]
    pub parent: Option<serde_json::Value>,

    #[serde(rename = "priority")]
    pub priority: Option<serde_json::Value>,

    #[serde(rename = "due_date")]
    pub due_date: Option<String>,

    #[serde(rename = "start_date")]
    pub start_date: Option<String>,

    #[serde(rename = "points")]
    pub points: Option<serde_json::Value>,

    #[serde(rename = "time_estimate")]
    pub time_estimate: Option<serde_json::Value>,

    #[serde(rename = "custom_fields")]
    pub custom_fields: Vec<Option<serde_json::Value>>,

    #[serde(rename = "dependencies")]
    pub dependencies: Vec<Option<serde_json::Value>>,

    #[serde(rename = "linked_tasks")]
    pub linked_tasks: Vec<Option<serde_json::Value>>,

    #[serde(rename = "team_id")]
    pub team_id: String,

    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "permission_level")]
    pub permission_level: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Creator {
    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "username")]
    pub username: String,

    #[serde(rename = "color")]
    pub color: String,

    #[serde(rename = "initials")]
    pub initials: Option<String>,

    #[serde(rename = "email")]
    pub email: String,

    #[serde(rename = "profilePicture")]
    pub profile_picture: Option<serde_json::Value>,
}
