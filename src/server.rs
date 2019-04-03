#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod store;
mod parse;
mod util;
mod view;

use crate::store::Store;
use crate::store::sale::Filial;
use crate::util::Month;
use rocket::Request;
use rocket::State;
use rocket::request::FromParam;
use rocket::http::RawStr;
use chrono::Local;


impl<'r> FromParam<'r> for Month {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let number = match param.as_str().parse::<u8>() {
            Err(_) => return Err(param),
            Ok(n) => n,
        };
        if let Some(m) = Month::from(number) {
            Ok(m)
        } else {
            Err(param)
        }
    }
}

impl<'r> FromParam<'r> for Filial {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let number = match param.as_str().parse::<u8>() {
            Err(_) => return Err(param),
            Ok(n) => n,
        };
        if let Some(f) = Filial::from_u8(number) {
            Ok(f)
        } else {
            Err(param)
        }
    }
}

#[get("/")]
fn index() -> &'static str {
"   Welcome to the LI3 API!
    command 2 <leter>
    command 3 <month> <product>
    command 4
    command 4 <filial>
    command 5
    command 6
    command 7 <client>
    command 8 <from> <to>
    command 9 <product> <filial> <promotion>
    command 10 <client> <month>
    command 11 <n>
    command 12 <client>

    Notes:
    <promotion> is 'true' or 'false'
    "
}

#[get("/2/<leter>")]
fn query2(store: State<Store>, leter: String) -> String {
    eprintln!("[{:?}] Running query2/{}", Local::now(), leter);
    let a = leter.chars().next().unwrap();
    view::list_by_first_letter(store.list_by_first_letter(a))
}

#[get("/3/<month>/<client>")]
fn query3(store: State<Store>, month: Month, client: String) -> String {
    eprintln!("[{:?}] Running query3/{}/{}", Local::now(), month, client);
    view::total_billed(store.total_billed(month.into(), client))
}

#[get("/4")]
fn query4(store: State<Store>) -> String {
    eprintln!("[{:?}] Running query4", Local::now());
    view::never_bought(store.never_bought())
}

#[get("/4/<filial>")]
fn query4_filial(store: State<Store>, filial: Filial) -> String {
    eprintln!("[{:?}] Running query4/{}", Local::now(), filial);
    view::never_bought(store.never_bought_filial(filial.into()))
}

#[get("/5")]
fn query5(store: State<Store>) -> String {
    eprintln!("[{:?}] Running query5", Local::now());
    view::buyers_in_all_filials(store.buyers_in_all_filials())
}

#[get("/6")]
fn query6(store: State<Store>) -> String {
    eprintln!("[{:?}] Running query6", Local::now());
    let buyers = store.n_buyers_without_purchases();
    let products = store.n_never_bought();
    view::never_bought_never_purchased(buyers, products)
}

#[get("/7/<client>")]
fn query7(store: State<Store>, client: String) -> String {
    eprintln!("[{:?}] Running query7/{}", Local::now(), client);
    view::year_purchases(store.year_purchases(client))
}

#[get("/8/<from>/<to>")]
fn query8(store: State<Store>, from: Month, to: Month) -> String {
    eprintln!("[{:?}] Running query8/{}/{}", Local::now(), from, to);
    view::total_billed_between(store.total_billed_between(from.into(), to.into()))
}

#[get("/9/<product>/<filial>/<promotion>")]
fn query9(store: State<Store>, product: String, filial: Filial, promotion: bool) -> String {
    eprintln!("[{:?}] Running query9/{}/{}/{}", Local::now(), product, filial, promotion);
    view::product_buyers(store.product_buyers(&product, filial.into(), promotion))
}

#[get("/10/<client>/<month>")]
fn query10(store: State<Store>, client: String, month: Month) -> String {
    eprintln!("[{:?}] Running query10/{}/{}", Local::now(), client, month);
    view::top_purchases(store.top_purchases(&client, month.into()))
}

#[get("/11/<n>")]
fn query11(store: State<Store>, n: usize) -> String {
    eprintln!("[{:?}] Running query11/{}", Local::now(), n);
    view::top_sold_products(store.top_sold_products(n, false))
}

#[get("/12/<client>")]
fn query12(store: State<Store>, client: String) -> String {
    eprintln!("[{:?}] Running query12/{}", Local::now(), client);
    view::top_expense(store.top_expense(&client))
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
        .mount("/", routes![index,query2,query3,query4,query4_filial,query5,query6,query7,query8,query9,query10,query11,query12])
        .register(catchers![catch404])
        .manage(store)
        .launch();
    Ok(())
}

