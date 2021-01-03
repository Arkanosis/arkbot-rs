pub struct Page {
    pub namespace: i32,
    pub title: String,
    pub text: Option<String>,
    pub target: Option<String>,
    pub username: Option<String>,
    pub timestamp: Option<String>,
}
