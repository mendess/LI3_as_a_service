#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod store;
mod parse;
mod util;

use crate::store::Store;
use crate::store::sale::Filial;
use crate::util::Month;
use rocket::Request;
use rocket::State;
use rocket::request::FromParam;
use rocket::http::RawStr;

#[derive(Debug)]
struct MonthWrapper(Month);

impl<'r> FromParam<'r> for MonthWrapper {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let number = match param.as_str().parse::<u8>() {
            Err(_) => return Err(param),
            Ok(n) => n,
        };
        if number > 0 && number < 13 {
            Ok(MonthWrapper(Month::from(number)))
        } else {
            Err(param)
        }
    }
}

impl Into<Month> for MonthWrapper {
    fn into(self: MonthWrapper) -> Month {
        self.0
    }
}

#[derive(Debug)]
struct FilialWrapper(Filial);

impl<'r> FromParam<'r> for FilialWrapper {
    type Error = &'r RawStr;

    fn from_param(param: &'r RawStr) -> Result<Self, Self::Error> {
        let number = match param.as_str().parse::<u8>() {
            Err(_) => return Err(param),
            Ok(n) => n,
        };
        if number > 0 && number < 4 {
            Ok(FilialWrapper(Filial::from(number)))
        } else {
            Err(param)
        }
    }
}

impl Into<Filial> for FilialWrapper {
    fn into(self: FilialWrapper) -> Filial {
        self.0
    }
}

#[get("/")]
fn index() -> &'static str {
"   Welcome to the LI3 API!
    command 2 <leter>
    command 3 <month> <client>
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
    eprintln!("Running query2/{}", leter);
    let l = store.list_by_first_letter(leter.chars().next().unwrap());
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}", l.len());
    response
}

#[get("/3/<month>/<client>")]
fn query3(store: State<Store>, month: MonthWrapper, client: String) -> String {
    eprintln!("Running query3/{}/{}", month.0, client);
    let l = store.total_billed(month.into(), client);
    format!("{:#?}", l)
}

#[get("/4")]
fn query4(store: State<Store>) -> String {
    eprintln!("Running query4");
    let l = store.never_bought();
    let mut response = String::new();
    for p in l.1.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}\n", l.0);
    response
}

#[get("/4/<filial>")]
fn query4_filial(store: State<Store>, filial: FilialWrapper) -> String {
    eprintln!("Running query4/{}", filial.0);
    let l = store.never_bought_filial(filial.into());
    let mut response = String::new();
    for p in l.1.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}\n", l.0);
    response
}

#[get("/5")]
fn query5(store: State<Store>) -> String {
    eprintln!("Running query5");
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
    eprintln!("Running query7/{}", client);
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
fn query8(store: State<Store>, from: MonthWrapper, to: MonthWrapper) -> String {
    eprintln!("Running query8/{}/{}", from.0, to.0);
    format!("{:?}", store.total_billed_between(from.into(), to.into()))
}

#[get("/9/<product>/<filial>/<promotion>")]
fn query9(store: State<Store>, product: String, filial: FilialWrapper, promotion: bool) -> String {
    eprintln!("Running query9/{}/{}/{}", product, filial.0, promotion);
    let l = store.product_buyers(&product, filial.into(), promotion);
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    response
}

#[get("/10/<client>/<month>")]
fn query10(store: State<Store>, client: String, month: MonthWrapper) -> String {
    eprintln!("Running query10/{}/{}", client, month.0);
    let l = store.top_purchases(&client, month.into());
    let mut response = String::new();
    for p in l {
        response += &format!("{} : {}", p.0, p.1);
    }
    response
}

#[get("/11/<n>")]
fn query11(store: State<Store>, n: usize) -> String {
    eprintln!("Running query11/{}", n);
    let l = store.top_sold_products(n, false);
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{:?}\n", p);
    }
    response
}

#[get("/12/<client>")]
fn query12(store: State<Store>, client: String) -> String {
    eprintln!("Running query12/{}", client);
    let l = store.top_expense(&client);
    let mut response = String::new();
    if let Some(p) = l.0 { response += &format!("{}\n", p); }
    if let Some(p) = l.1 { response += &format!("{}\n", p); }
    if let Some(p) = l.2 { response += &format!("{}\n", p); }
    response
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

