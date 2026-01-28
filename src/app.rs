use crate::{
    api::{get_all_city, get_city},
    helper::{output, update_client},
    models::Client,
    utils::{clear_line, spinner_loop},
};

pub async fn run(input: &str, time: String) {
    let result = tokio::select! {
        res = get_city(input) => res,
        _ = spinner_loop("Loading get city ") => unreachable!(),
    };

    clear_line(0, 0);

    let mut client = Client::default();

    update_client(&mut client, result);

    clear_line(0, 0);

    output(client, &time).await;
}

pub async fn run_by_id(id: u16, time: String) {
    let client = Client {
        id: Some(id.to_string()),
        is_multiple: false
    };

    output(client, &time).await
}

pub async fn _list() {
    let result = tokio::select! {
        res = get_all_city() => res,
        _ = spinner_loop("Loading list all possible city ") => unreachable!(),
    };

    match result {
        Ok(resp) => {
            resp.data.iter().for_each(|f| {
                for i in f {
                    println!("ID: {} - {}", i.id, i.lokasi);
                }
            });
        }
        Err(e) => {
            println!("Error: {e}");
        }
    }
}

