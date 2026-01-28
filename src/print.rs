use crate::models::Jadwal;

fn prints(jadwal: Jadwal) {
    println!("Subuh: {}", jadwal.jadwal.subuh);
    println!("Dzuhur: {}", jadwal.jadwal.dzuhur);
    println!("Ashar: {}", jadwal.jadwal.ashar);
    println!("Maghrib: {}", jadwal.jadwal.maghrib);
    println!("Isya: {}", jadwal.jadwal.isya);
}

pub fn print_jadwal(jadwal: Jadwal, is_multiple: bool) {
    if is_multiple {
        println!("Jadwal Sholat {}", jadwal.lokasi);
        prints(jadwal);
    } else {
        if jadwal.lokasi.contains("KAB.") {
            let lokasi_formated = jadwal.lokasi.replace("KAB. ", "");
            println!("Jadwal Sholat {}", lokasi_formated);
            prints(jadwal);
        } else {
            let lokasi_formated = jadwal.lokasi.replace("KOTA ", "");
            println!("Jadwal Sholat {}", lokasi_formated);
            prints(jadwal);
        }
    }
}
