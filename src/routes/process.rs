use axum::response::Html;

pub async fn add_process() -> Html<&'static str> {
    Html("<span>你好</span>")
}
