use std::collections::HashMap;
use reqwest::Client;
use scraper::{ElementRef, Html, Selector};

use crate::parse::models::{train::Train, ticket::Ticket};

async fn select(el: &ElementRef<'_>, str_parse: &str) -> String {
    let sel = Selector::parse(str_parse).unwrap();
    el.select(&sel)
        .map(|el| el.text().filter(|s| !s.trim().is_empty()).collect::<Vec<_>>().join(" "))
        .collect::<Vec<_>>()
        .join(" ").trim().to_string()
}

async fn parse_selector() -> Selector {
    Selector::parse("div.sch-table__body.js-sort-body").unwrap()
}

async fn get_train_number(el: &ElementRef<'_>) -> String{
    select(&el, "span.train-number").await
}

async fn get_start_station(el: &ElementRef<'_>) -> String {
    select(&el, "div.sch-table__station.train-from-name").await
}

async fn get_route(el: &ElementRef<'_>) -> String {
    select(&el, "span.train-route").await
}

async fn get_end_station(el: &ElementRef<'_>) -> String {
    select(&el, "div.sch-table__station.train-to-name").await
}

async fn get_start_time(el: &ElementRef<'_>) -> String {
    select(&el, "div.sch-table__time.train-from-time").await
}
async fn get_end_time(el: &ElementRef<'_>) -> String {
    select(&el, "div.sch-table__time.train-to-time").await
}

async fn parse_tickets() -> Selector {
    Selector::parse("div.sch-table__tickets").unwrap()
}

async fn get_ticket_name(el: &ElementRef<'_>) -> String {
    select(&el, "div.sch-table__t-name").await
}

async fn get_awailable_tickets(el: &ElementRef<'_>) -> String {
    select(&el, "a.sch-table__t-quant.js-train-modal.dash").await
}

async fn get_ticket_cost(el: &ElementRef<'_>) -> String {
    select(&el, "span.ticket-cost").await
}
pub async fn get_all_trains(cl: &Client, date: &str) -> HashMap<String, Train> {
    let url = format!("https://pass.rw.by/ru/route/?from=%D0%9C%D0%B8%D0%BD%D1%81%D0%BA-%D0%9F%D0%B0%D1%81%D1%81%D0%B0%D0%B6%D0%B8%D1%80%D1%81%D0%BA%D0%B8%D0%B9&from_exp=2100001&from_esr=140210&to=%D0%9E%D1%80%D1%88%D0%B0-%D0%A6%D0%B5%D0%BD%D1%82%D1%80%D0%B0%D0%BB%D1%8C%D0%BD%D0%B0%D1%8F&to_exp=2100170&to_esr=166403&date={date}&type=1");
    let mut map = HashMap::new();
    let response = cl
        .get(url)
        .send().await
        .unwrap()
        .text()
        .await
        .unwrap();

    let binding = Html::parse_document(&response);
    let sel = parse_selector().await;

    let rows_data = binding
        .select(&sel).next().unwrap();

    for train_el in rows_data.child_elements() {

        let number= get_train_number(&train_el).await;
        if number == "" {
            continue;
        }

        let start_station = get_start_station(&train_el).await;

        let route = get_route(&train_el).await;

        let end_station = get_end_station(&train_el).await;

        let start_time = get_start_time(&train_el).await;

        let end_time = get_end_time(&train_el).await;


        println!("{} {}", number, route);
        println!("{} - {}", start_station, end_station);
        println!("{} - {}", start_time, end_time);


        let tickets_sel = parse_tickets().await;
        let tickets = train_el.select(&tickets_sel).next().unwrap();

        let ticket_cost = get_ticket_cost(&train_el).await;
        let tickets_cost_vec: Vec<&str> = ticket_cost.trim().split(" ").collect();
        let mut vec_tickets: Vec<Ticket> = Vec::new();

        for (index,ticket_el) in tickets.child_elements().into_iter().enumerate() {

            let ticket_seat_type = get_ticket_name(&ticket_el)
                .await;

            let awailable_seats = get_awailable_tickets(&ticket_el)
                .await
                .to_owned()
                .parse::<u8>()
                .unwrap_or_else(|_| 0);
            let ticket_cost = tickets_cost_vec[index]
                .replace(",", ".")
                .parse::<f32>()
                .unwrap_or_else(|_| 0.0);

            vec_tickets.push(Ticket::new(ticket_seat_type, awailable_seats, ticket_cost));
            println!("{:?}", vec_tickets[index]);
        }
        let train = Train::new(number.clone(), route, start_station, end_station, start_time, end_time, vec_tickets);
        map.insert(number.clone(), train);
        println!("-----------------");
    }
    map
}
