#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate rocket_contrib;

// #[rocket_contrib::database("db")]
#[database("mongodb")]
struct DatabaseConnection(rocket_contrib::databases::mongodb::db::Database);

// root@a749891c0677:/workspace# cargo run
//     Finished dev [unoptimized + debuginfo] target(s) in 0.70s
//      Running `target/debug/hello-rust-rocket-mongo`
// 🔧 Configured for development.
//     => address: 0.0.0.0
//     => port: 8001
//     => log: normal
//     => workers: 24
//     => secret key: generated
//     => limits: forms = 32KiB
//     => keep-alive: 5s
//     => tls: disabled
//     => [extra] databases: { mongodb = { url = "mongodb://root:password@mongo/mydb" } }

// use rocket_contrib::databases::diesel;
// use rocket_contrib::databases::mongodb;

// // #[database("sqlite_logs")]
// #[database("mongodb")]
// struct LogsDbConn(mongodb::Client);
// // struct LogsDbConn(diesel::SqliteConnection);
// // struct LogsDbConn(mongodb::Mongo);

// /// Connections for the SQLite backend. Unlike other backends, "connection URLs"
// /// for SQLite are file paths, [URIs](https://sqlite.org/uri.html), or special
// /// identifiers like `:memory:`.
// #[allow(missing_debug_implementations)]
// pub struct SqliteConnection {
//     statement_cache: StatementCache<Sqlite, Statement>,
//     raw_connection: RawConnection,
//     transaction_manager: AnsiTransactionManager,
// }

#[get("/")]
fn index (connection: DatabaseConnection) -> String {
    format!("{:?}", connection.0.client)
}

fn main() {
    rocket::ignite()
        .attach(DatabaseConnection::fairing())
        .mount("/", routes![index])
        .launch();
}

// 🔧 Configured for development.
// => address: 0.0.0.0
// => port: 8001
// => log: normal
// => workers: 24
// => secret key: generated
// => limits: forms = 32KiB
// => keep-alive: 5s
// => tls: disabled
// => [extra] databases: { mongodb = { url = "mongodb://usr:pass@mongo:27017/hello" } }
// 🛰  Mounting /:
// => GET / (index)
// 🚀 Rocket has launched from http://0.0.0.0:8001
// GET /:
// => Matched: GET / (index)
// => Outcome: Success
// => Response succeeded.
//
// root@341784d8367f:/workspace# curl localhost:8001/
// ClientInner { read_preference: ReadPreference { mode: Primary, tag_sets: [] }, write_concern: WriteConcern { w: 1, w_timeout: 0, j: false, fsync: false }, req_id: 9, topology: Topology { config: ConnectionString { hosts: [Host { host_name: "mongo", ipc: "", port: 27017 }], string: None, user: None, password: None, database: Some("test"), collection: None, options: None }, description: RwLock { data: TopologyDescription { topology_type: Single, set_name: "", servers: "HashMap<Host, Server> { .. }", heartbeat_frequency_ms: 10000, local_threshold_ms: 15, server_selection_timeout_ms: 30000, max_election_id: None, compatible: true, max_set_version: None, compat_error: "", stream_connector: "StreamConnector { .. }" } } }, listener: "Listener { .. }", log_file: None }