#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod store;
mod parse;
mod util;

use crate::store::Store;
use crate::util::Month;
use rocket::Request;

#[get("/")]
fn index() -> &'static str {
"   Welcome to the LI3 API!
    /query2/<leter>
    /query3/<month>/<client>
    /query4
    /query4/<filial>
    /query5
    /query6
    /query7/<client>
    /query8/<from>/<to>
    /query9/<product>/<filial>/<promotion>
    /query10
    /query11/<n>
    /query12
    "
}

#[get("/query2/<leter>")]
fn query2(leter: String) -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let l = store.list_by_first_letter(leter.chars().next().unwrap()); //STORE not in scope
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    format!("{}", response)
}

#[get("/query3/<month>/<client>")]
fn query3(month: u8, client: String) -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let l = store.total_billed(Month::from(month), client);
    format!("{:#?}", l)
}

#[get("/query4")]
fn query4() -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let l = store.never_bought();
    let mut response = String::new();
    for p in l.1.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}\n", l.0);
    format!("{}", response)
}

#[get("/query4/<filial>")]
fn query4_filial(filial: u8) -> String {
    format!("Not done yet!\n")
}

#[get("/query5")]
fn query5() -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let l = store.buyers_in_all_filials();
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    format!("{}", response)
}

#[get("/query6")]
fn query6() -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let buyers = store.n_buyers_without_purchases();
    let products = store.n_never_bought();
    format!("clients: {} | products: {}", buyers, products)
}

#[get("/query7/<client>")]
fn query7(client: String) -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let table = store.year_purchases(client);
    let mut response = String::new();
    response += "       | Jan | Fev | Mar | Apr | May | Jun | Jul | Aug | Sep | Oct | Nov | Dez |\n";
    response += "-------+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+\n";
    response +=  " One   |\n";
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

#[get("/query8/<from>/<to>")]
fn query8(from: u8, to: u8) -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    format!("{:?}", store.total_billed_between(Month::from(from), Month::from(to)))
}

#[get("/query9/<product>/<filial>/<promotion>")]
fn query9(product: String, filial: u8, promotion: bool) -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let l = store.product_buyers(&product, store::sale::Filial::from(filial), promotion);
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{}\n", p);
    }
    format!("{}", response)
}

#[get("/query10")]
fn query10() -> String {
    format!("Not done yet!")
}

#[get("/query11/<n>")]
fn query11(n: usize) -> String {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store).unwrap();
    parse::load_products("./db/Produtos.txt", &mut store).unwrap();
    parse::load_sales("./db/Vendas_1M.txt", &mut store).unwrap();
    let l = store.top_sold_products(n, false);
    let mut response = String::new();
    for p in l.iter() {
        response += &format!("{:?}\n", p);
    }
    format!("{}", response)
}

#[catch(404)]
fn catch404(_req: &Request) -> String {
    format!("Not a valid query!")
}

#[get("/query12")]
fn query12() -> String {
    format!("Not done yet!\n")
}

fn main() -> std::io::Result<()>{
    rocket::ignite().mount("/", routes![index,query2,query3,query4,query5,query6,query7,query8,query9,query10,query11,query12]).register(catchers![catch404]).launch();
    Ok(())
}

