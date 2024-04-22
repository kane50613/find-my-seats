use cached::proc_macro::cached;
use serde::Deserialize;

use crate::{error::MyError, Movie};

#[derive(Debug, Deserialize)]
struct DicItem {
    #[serde(rename = "strText")]
    str_text: String,
    #[serde(rename = "strValue")]
    str_value: String,
}

#[cached(time = 60)]
pub async fn list_movies() -> Result<Vec<Movie>, MyError> {
    let result =
        reqwest::get("https://www.vscinemas.com.tw/vsweb/api/GetLstDicFilm?area=1").await?;

    let items: Vec<DicItem> = result.json().await?;

    Ok(items
        .iter()
        .map(|item| Movie {
            id: item.str_value.clone(),
            title: item.str_text.clone(),
        })
        .collect())
}
