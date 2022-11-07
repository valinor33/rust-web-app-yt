use gloo_net::http::Request;
use serde::Deserialize;

pub async fn search_youtube(text_to_search: String) -> Result<VideoItem, gloo_net::Error> {
    let query_url = format!(
        "https://youtube.googleapis.com/youtube/v3/search?part=id%2Csnippet&q={}",
        text_to_search
    );

    let mut auth: String = String::from("Bearer ");
    auth.push_str("API KEY");

    let response = Request::get(&query_url)
        .header("Authorization", &auth)
        .send()
        .await?;
    let search_result = response.json::<SearchResult>().await?;
    let empty_video = build_empty_video();
    let video = match search_result.items.first() {
        Some(video) => video,
        None => &empty_video,
    };

    web_sys::console::log_1(&text_to_search.into());
    Ok(video.clone())
}

fn build_empty_video() -> VideoItem {
    VideoItem {
        id: VideoItemId {
            kind: "".to_string(),
            videoId: "".to_string(),
        },
        snippet: VideoSnippet {
            title: "".to_string(),
            description: "".to_string(),
        },
    }
}

#[derive(Clone, Deserialize)]
struct SearchResult {
    items: Vec<VideoItem>,
}

#[derive(Clone, Deserialize)]
pub struct VideoItem {
    pub id: VideoItemId,
    pub snippet: VideoSnippet,
}

#[derive(Clone, Deserialize)]
pub struct VideoItemId {
    pub kind: String,
    pub videoId: String,
}

#[derive(Clone, Deserialize)]
pub struct VideoSnippet {
    pub title: String,
    pub description: String,
}
