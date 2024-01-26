#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub id: String,
    pub about: String,
    pub src: String,
    pub pre_run: Vec<Task>,
    pub post_run: Vec<Task>,
}
