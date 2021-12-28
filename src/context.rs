use tokio::task_local;

task_local! {
    pub static LOG_ID: String; // RefCell<Arc<>>?
    pub static LANG: String;
}