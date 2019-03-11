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
        eprintln!("{:?}", now.elapsed());
    }
    {// 2
        let now = Instant::now();
        for p in store.list_by_first_letter('b').iter() {
            println!("{}", p);
        }
        eprintln!("{:?}", now.elapsed());
    }
    { // 3
        let now = Instant::now();
        println!("{:#?}", store.total_billed(Month::from(10), "CC1684".to_string()));
        eprintln!("{:?}", now.elapsed());
    }
    {// 4
        let now = Instant::now();
        let never_bought = store.never_bought();
        for p in never_bought.1 {
            println!("{}", p.id());
        }
        println!("{}", never_bought.0);
        eprintln!("{:?}", now.elapsed());
    }
    {// 5
        let now = Instant::now();
        for p in store.buyers_in_all_filials() {
            println!("{}", p);
        }
        eprintln!("{:?}", now.elapsed());
    }
    {// 6
        let now = Instant::now();
        let buyers = store.n_buyers_without_purchases();
        let products = store.n_never_bought();
        println!("clients: {} | products: {}", buyers, products);
        eprintln!("{:?}", now.elapsed());
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
        eprintln!("{:?}", now.elapsed());
    }
    {// 8
        let now = Instant::now();
        println!("{:?}", store.total_billed_between(Month::from(1), Month::from(3)));
        eprintln!("{:?}", now.elapsed());
    }
    {// 9
    }
    {// 10
    }
    {// 11
    }
    {// 12
    }
    Ok(())
}
