use axum::{
    extract::{Path, Query},
    http::StatusCode,
    response::Json,
    routing::get,
    Router,
};
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize, Default, Clone, Copy)]
#[serde(rename_all = "lowercase")]
enum Lang {
    #[default]
    En,
    Am,
    Or,
}

impl Lang {
    fn as_code(&self) -> &'static str {
        match self {
            Lang::En => "en",
            Lang::Am => "am",
            Lang::Or => "or",
        }
    }
}

#[derive(Debug, Deserialize, Default)]
struct Params {
    #[serde(default)]
    lang: Lang,
}

async fn get_tin(
    Path(tin): Path<String>,
    Query(params): Query<Params>,
) -> Result<Json<Value>, (StatusCode, String)> {
    let url = format!(
        "https://etrade.gov.et/api/Registration/GetRegistrationInfoByTin/{tin}/{}",
        params.lang.as_code()
    );

    let resp = reqwest::Client::new()
        .get(&url)
        .header("accept", "application/json, text/plain, */*")
        .header("referer", format!("https://etrade.gov.et/business-license-checker?tin={tin}"))
        .header("user-agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
            AppleWebKit/537.36 (KHTML, like Gecko) Chrome/148.0.0.0 Safari/537.36")
        .send()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    let data = resp
        .json::<Value>()
        .await
        .map_err(|e| (StatusCode::BAD_GATEWAY, e.to_string()))?;

    Ok(Json(data))
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/tin/{tin}", get(get_tin));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();
    println!("listening on http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
