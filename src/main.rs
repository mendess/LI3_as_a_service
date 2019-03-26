mod store;
mod parse;
mod util;

use crate::store::Store;
use crate::util::Month;

use std::time::Instant;

fn main() -> std::io::Result<()> {
    let mut store = Store::new();
    {//1
        let now = Instant::now();
        parse::load_clients("./db/Clientes.txt", &mut store)?;
        parse::load_products("./db/Produtos.txt", &mut store)?;
        parse::load_sales("./db/Vendas_1M.txt", &mut store)?;
        store.serialize()?;
        eprintln!("Query  1: {:?}", now.elapsed());
    }
    {// 2
        let now = Instant::now();
        let list = store.list_by_first_letter('a');
        eprintln!("Query  2: {:?}", now.elapsed());
        for p in list.iter() {
            println!("{}", p);
        }
    }
    { // 3
        let now = Instant::now();
        println!("{:#?}", store.total_billed(Month::from(10), "CC1684".to_string()));
        eprintln!("Query  3: {:?}", now.elapsed());
    }
    {// 4
        let now = Instant::now();
        let never_bought = store.never_bought();
        for p in never_bought.1 {
            println!("{}", p.id());
        }
        println!("{}", never_bought.0);
        eprintln!("Query  4: {:?}", now.elapsed());
    }
    {// 5
        let now = Instant::now();
        for p in store.buyers_in_all_filials() {
            println!("{}", p);
        }
        eprintln!("Query  5: {:?}", now.elapsed());
    }
    {// 6
        let now = Instant::now();
        let buyers = store.n_buyers_without_purchases();
        let products = store.n_never_bought();
        println!("clients: {} | products: {}", buyers, products);
        eprintln!("Query  6: {:?}", now.elapsed());
    }
    {//7
        let now = Instant::now();
        let table = store.year_purchases(String::from("F2916"));
        println!("       | Jan | Fev | Mar | Apr | May | Jun | Jul | Aug | Sep | Oct | Nov | Dez |");
        println!("-------+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+");
        print!(  " One   |");
        for month in table.0.iter() {
            print!(" {:3} |", month);
        }
        println!("");
        print!(  " Two   |");
        for month in table.1.iter() {
            print!(" {:3} |", month);
        }
        println!("");
        print!(  " Three |");
        for month in table.2.iter() {
            print!(" {:3} |", month);
        }
        println!("");
        eprintln!("Query  7: {:?}", now.elapsed());
    }
    {// 8
        let now = Instant::now();
        println!("{:?}", store.total_billed_between(Month::from(1), Month::from(3)));
        eprintln!("Query  8: {:?}", now.elapsed());
    }
    {// 9
        let now = Instant::now();
        for p in store.product_buyers("AA1001", store::sale::Filial::One, true) {
            println!("{}", p);
        }
        eprintln!("Query  9: {:?}", now.elapsed());
    }
    {// 10
        let now = Instant::now();
        for p in store.top_purchases("A1234", Month::from(1)) {
            println!("{:?}", p);
        }
        eprintln!("Query  10: {:?}", now.elapsed());
    }
    {// 11
        let now = Instant::now();
        for ps in store.top_sold_products(10, false) {
            println!("{:?}", ps);
        }
        eprintln!("Query 11: {:?}", now.elapsed());
    }
    {// 12
        let now = Instant::now();
        let l = store.top_expense("A1234");
        let mut response = String::new();
        if let Some(p) = l.0 { response += &format!("{}\n", p); }
        if let Some(p) = l.1 { response += &format!("{}\n", p); }
        if let Some(p) = l.2 { response += &format!("{}\n", p); }
        print!("{}", response);
        eprintln!("Query 11: {:?}", now.elapsed());
    }
    Ok(())
}
