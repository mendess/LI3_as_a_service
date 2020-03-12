mod parse;
mod store;
mod util;
mod view;

use crate::store::Store;
use crate::util::Month;

use std::env;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let show_output = !env::args().any(|x| x == "-q" || x == "--quiet");
    let mut store = Store::new();
    {
        //1
        let now = Instant::now();
        parse::load_clients("./db/Clientes.txt", &mut store)?;
        parse::load_products("./db/Produtos.txt", &mut store)?;
        parse::load_sales("./db/Vendas_1M.txt", &mut store)?;
        store.serialize()?;
        eprintln!(
            "Query  1: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
    }
    {
        // 2
        let now = Instant::now();
        let list = store.list_by_first_letter('a');
        eprintln!(
            "Query  2: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::list_by_first_letter(list))
        };
    }
    {
        // 3
        let now = Instant::now();
        let total_billed = store.total_billed(Month::Oct, "CC1684");
        eprintln!(
            "Query  3: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::total_billed(total_billed))
        };
    }
    {
        // 4
        let now = Instant::now();
        let never_bought = store.never_bought();
        eprintln!(
            "Query  4: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::never_bought(never_bought))
        };
    }
    {
        // 5
        let now = Instant::now();
        let buyers = store.buyers_in_all_filials();
        eprintln!(
            "Query  5: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::buyers_in_all_filials(buyers))
        };
    }
    {
        // 6
        let now = Instant::now();
        let buyers = store.n_buyers_without_purchases();
        let products = store.n_never_bought();
        eprintln!(
            "Query  6: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::never_bought_never_purchased(buyers, products))
        };
    }
    {
        //7
        let now = Instant::now();
        let table = store.year_purchases("F2916");
        eprintln!(
            "Query  7: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::year_purchases(table))
        };
    }
    {
        // 8
        let now = Instant::now();
        let total_billed = store.total_billed_between(Month::Jan, Month::Mar);
        eprintln!(
            "Query  8: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::total_billed_between(total_billed))
        };
    }
    {
        // 9
        let now = Instant::now();
        let clients = store.product_buyers("AA10001", store::sale::Filial::One, true);
        eprintln!(
            "Query  9: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::product_buyers(clients))
        };
    }
    {
        // 10
        let now = Instant::now();
        let purchases = store.top_purchases("A1234", Month::Jan);
        eprintln!(
            "Query 10: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::top_purchases(purchases))
        };
    }
    {
        // 11
        let now = Instant::now();
        let top_prods = store.top_sold_products(10, false);
        eprintln!(
            "Query 11: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::top_sold_products(top_prods))
        };
    }
    {
        // 12
        let now = Instant::now();
        let l = store.top_expense("A1234");
        eprintln!(
            "Query 12: {:?}ms",
            now.elapsed().as_micros() as f64 / 1000.0
        );
        if show_output {
            println!("{}", view::top_expense(l))
        };
    }
    Ok(())
}
