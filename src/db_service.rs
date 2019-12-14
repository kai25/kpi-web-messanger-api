use futures::FutureExt;

use tokio::spawn;
use tokio_postgres::{Client, Error as PGError, NoTls};

pub struct DBBuilder {
    host: String,
    port: u16,
    dbname: String,
    username: String,
    password: String,
}

impl DBBuilder {
    pub fn new() -> DBBuilder {
        DBBuilder {
            host: "localhost".to_owned(),
            port: 5432,
            dbname: "db".to_owned(),
            username: "postgres".to_owned(),
            password: "postgres".to_owned(),
        }
    }

    pub fn set_host(&mut self, host: &str) {
        DBBuilder::check_value(host, "host");
        self.host = host.to_owned();
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_dbname(&mut self, dbname: &str) {
        DBBuilder::check_value(dbname, "dbname");
        self.dbname = dbname.to_owned();
    }

    pub fn set_username(&mut self, username: &str) {
        DBBuilder::check_value(username, "username");
        self.username = username.to_owned();
    }

    pub fn set_password(&mut self, password: &str) {
        DBBuilder::check_value(password, "password");
        self.password = password.to_owned();
    }

    fn check_value(value: &str, value_name: &str) {
        if value.len() == 0 {
            panic!(format!("{} length should be greater then 0", value_name));
        }
    }
}

pub struct DBService {
    pub client: Client,
}

impl DBService {
    pub async fn from_config(config: &DBBuilder) -> Result<DBService, PGError> {
        let config_line = "host=localhost user=postgres password=postgres dbname=chat-api";
        match tokio_postgres::connect(config_line, NoTls).await {
            Err(err) => return Err(err),
            Ok((db_client, connection)) => {
                let connection = connection.map(|res| {
                    if let Err(e) = res {
                        panic!(format!("Error during db initiating {}", e.to_string()));
                    }
                });

                spawn(connection);

                Ok(DBService { client: db_client })
            }
        }
    }
}
