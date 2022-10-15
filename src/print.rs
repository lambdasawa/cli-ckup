use std::fmt::Debug;

use prettytable::{row, Table};

use crate::{config, model};

pub fn print_by_cfg<T>(cfg: &config::Config, value: &T)
where
    T: ?Sized + serde::Serialize + IntoTable + Debug,
{
    println!("cfg: {:?}", cfg);

    match cfg.format {
        config::FormatConfig::Debug => println!("{:#?}", value),
        config::FormatConfig::JSON => print_json(value),
        config::FormatConfig::Table => print_table(value),
    }
}

fn print_json<T>(value: &T)
where
    T: ?Sized + serde::Serialize,
{
    println!("{}", serde_json::to_string_pretty(value).unwrap());
}

fn print_table<T>(value: &T)
where
    T: ?Sized + IntoTable,
{
    value.table().printstd();
}

pub trait IntoTable {
    fn table(&self) -> Table;
}

impl IntoTable for Vec<model::User> {
    fn table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(row!["id", "username", "email"]);

        self.into_iter().for_each(|v| {
            table.add_row(row![v.id, v.username, v.email]);
        });

        table
    }
}

impl IntoTable for Vec<model::Workspace> {
    fn table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(row!["id", "name"]);

        self.into_iter().for_each(|v| {
            table.add_row(row![v.id, v.name]);
        });

        table
    }
}

impl IntoTable for Vec<model::Space> {
    fn table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(row!["id", "name", "private", "archived"]);

        self.into_iter().for_each(|v| {
            table.add_row(row![v.id, v.name, v.private, v.archived]);
        });

        table
    }
}

impl IntoTable for Vec<model::List> {
    fn table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(row!["id", "name", "task_count", "archived"]);

        self.into_iter().for_each(|v| {
            table.add_row(row![v.id, v.name, v.task_count, v.archived]);
        });

        table
    }
}

impl IntoTable for Vec<model::Task> {
    fn table(&self) -> Table {
        let mut table = Table::new();

        table.add_row(row![
            "id",
            "name",
            "status",
            "assignees",
            "date_created",
            "due_date"
        ]);

        self.into_iter().for_each(|v| {
            table.add_row(row![
                v.id,
                v.name,
                v.status.status,
                v.assignees
                    .iter()
                    .map(|a| a.username.clone())
                    .collect::<Vec<_>>()
                    .join(", "),
                v.date_created,
                v.due_date.as_ref().unwrap_or(&"".to_string()),
            ]);
        });

        table
    }
}
