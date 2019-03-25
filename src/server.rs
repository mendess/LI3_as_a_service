#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod store;
mod parse;
mod util;

use crate::store::Store;
use crate::util::Month;
use rocket::Request;
use rocket::State;

#[get("/")]
fn index() -> &'static str {
"   Welcome to the LI3 API!
    lixo3 2 <leter>
    lixo3 3 <month> <client>
    lixo3 4
    lixo3 4 <filial>
    lixo3 5
    lixo3 6
    lixo3 7 <client>
    lixo3 8 <from> <to>
    lixo3 9 <product> <filial> <promotion>
    lixo3 10 <client> <month>
    lixo3 11 <n>
    lixo3 12
    "
}

#[get("/2/<leter>")]
fn query2(store: State<Store>, leter: String) -> String {
    let l = store.list_by_first_letter(leter.chars().next().unwrap());
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}", l.len());
    response
}

#[get("/3/<month>/<client>")]
fn query3(store: State<Store>, month: u8, client: String) -> String {
    let l = store.total_billed(Month::from(month), client);
    format!("{:#?}", l)
}

#[get("/4")]
fn query4(store: State<Store>) -> String {
    let l = store.never_bought();
    let mut response = String::new();
    for p in l.1.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}\n", l.0);
    response
}

#[get("/4/<filial>")]
fn query4_filial(_store: State<Store>, filial: u8) -> String {
    format!("Not done yet!\n")
}

#[get("/5")]
fn query5(store: State<Store>) -> String {
    let l = store.buyers_in_all_filials();
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    response
}

#[get("/6")]
fn query6(store: State<Store>) -> String {
    let buyers = store.n_buyers_without_purchases();
    let products = store.n_never_bought();
    format!("clients: {} | products: {}", buyers, products)
}

#[get("/7/<client>")]
fn query7(store: State<Store>, client: String) -> String {
    let table = store.year_purchases(client);
    let mut response = String::new();
    response += "       | Jan | Fev | Mar | Apr | May | Jun | Jul | Aug | Sep | Oct | Nov | Dez |\n";
    response += "-------+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+\n";
    response += " One   |";
    for month in table.0.iter() {
        response += &format!(" {:3} |", month);
    }
    response += "\n";
    response += " Two   |";
    for month in table.1.iter() {
        response += &format!(" {:3} |", month);
    }
    response += "\n";
    response += " Three |";
    for month in table.2.iter() {
        response += &format!(" {:3} |", month);
    }
    response += "\n";
    response
}

#[get("/8/<from>/<to>")]
fn query8(store: State<Store>, from: u8, to: u8) -> String {
    format!("{:?}", store.total_billed_between(Month::from(from), Month::from(to)))
}

#[get("/9/<product>/<filial>/<promotion>")]
fn query9(store: State<Store>, product: String, filial: u8, promotion: bool) -> String {
    let l = store.product_buyers(&product, store::sale::Filial::from(filial), promotion);
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    response
}

#[get("/10/<client>/<month>")]
fn query10(store: State<Store>, client: String, month: u8) -> String {
    let l = store.top_purchases(&client, Month::from(month));
    let mut response = String::new();
    for p in l {
        response += &format!("{} : {}", p.0, p.1);
    }
    response
}

#[get("/11/<n>")]
fn query11(store: State<Store>, n: usize) -> String {
    let l = store.top_sold_products(n, false);
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{:?}\n", p);
    }
    response
}

#[get("/12")]
fn query12(_store: State<Store>) -> String {
    format!("Not done yet!\n")
}

#[catch(404)]
fn catch404(_req: &Request) -> String {
    format!("Not a valid query!")
}

fn main() -> std::io::Result<()>{
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    rocket::ignite()
        .mount("/", routes![index,query2,query3,query4,query5,query6,query7,query8,query9,query10,query11,query12])
        .register(catchers![catch404])
        .manage(store)
        .launch();
    Ok(())
}

