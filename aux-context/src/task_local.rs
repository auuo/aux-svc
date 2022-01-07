use tokio::task_local;

task_local! {
    pub static LOG_ID: String;
    pub static LANG: String;
}