use crate::{
    api::{get_city},
    helper::{output, update_client},
    models::Client,
    utils::{clear_line, spinner_loop},
};

pub async fn run(input: &str, time: String, make_default: bool) {
    let result = tokio::select! {
        res = get_city(input) => res,
        _ = spinner_loop("Loading get city ") => unreachable!(),
    };

    clear_line(0, 0);

    let mut client = Client::default();

    update_client(&mut client, result);

    clear_line(0, 0);

    output(client, &time, make_default).await;
}

pub async fn run_by_id(id: u16, time: String, make_default: bool) {
    let client = Client {
        id: Some(id.to_string()),
        is_multiple: false
    };

    output(client, &time, make_default).await
}
