use crate::{error::ApiError, models::{ApiResponse, JadwalResponse}};

pub async fn get_city(city: &str) -> Result<ApiResponse, ApiError> {
    let url = format!("https://api.myquran.com/v2/sholat/kota/cari/{}", city);
    let resp = reqwest::get(url).await?.json::<ApiResponse>().await?;

    Ok(resp)
}

pub async fn get_jadwal(city_id: &str, time: &str) -> Result<JadwalResponse, ApiError>{
    let url = format!("https://api.myquran.com/v2/sholat/jadwal/{}/{}", city_id, time);
    let resp = reqwest::get(url)
        .await?
        .json::<JadwalResponse>()
        .await?;

    Ok(resp)
}

pub async fn get_jadwal2(city_id: String, time: String) -> Result<JadwalResponse, Box<dyn std::error::Error + Send + Sync>>{
    let url = format!("https://api.myquran.com/v2/sholat/jadwal/{}/{}", city_id, time);
    let resp = reqwest::get(url)
        .await?
        .json::<JadwalResponse>()
        .await?;

    Ok(resp)
}

pub async fn get_all_city() -> Result<ApiResponse, Box<dyn std::error::Error>> {
    let resp = reqwest::get("https://api.myquran.com/v2/sholat/kota/semua")
        .await?
        .json::<ApiResponse>()
        .await?;

    Ok(resp)
}
