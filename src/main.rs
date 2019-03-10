mod store;
mod parse;
mod util;

use crate::store::Store;
use crate::util::Month;

fn main() -> std::io::Result<()> {
    let mut store = Store::new();
    parse::load_clients("./db/Clientes.txt", &mut store)?;
    parse::load_products("./db/Produtos.txt", &mut store)?;
    parse::load_sales("./db/Vendas_1M.txt", &mut store)?;
    for p in store.list_by_first_letter('b').iter() {
        println!("{}", p);
    }
    println!("{:#?}", store.total_billed(Month::from(10), "CC1684".to_string()));
    Ok(())
}
