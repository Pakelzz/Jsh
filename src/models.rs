#![allow(unused)]

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub status: bool,
    pub data: Option<Vec<Kota>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Kota {
    pub id: String,
    pub lokasi: String,
}

#[derive(Debug, Deserialize)]
pub struct JadwalResponse {
    pub status: bool,
    pub data: Option<Jadwal>
}

#[derive(Debug, Deserialize)]
pub struct Jadwal {
    pub lokasi: String,
    pub jadwal: Sholat
}

#[derive(Debug, Deserialize)]
pub struct Sholat {
    pub subuh: String,
    pub dzuhur: String,
    pub ashar: String,
    pub maghrib: String,
    pub isya: String
}

#[derive(Debug, Default)]
pub struct Client {
    pub id: Option<String>,
    pub is_multiple: bool,
    pub error: Option<String>
}
