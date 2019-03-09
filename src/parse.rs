use crate::store::Store;
use crate::store::product::Product;
use crate::store::client::Client;
use crate::store::sale::Sale;

use std::fs::File;
use std::io::{self, BufReader, BufRead};

#[allow(dead_code)]
pub fn load_products(file: &str, store: &mut Store) -> io::Result<()> {
    let file = BufReader::new(File::open(file)?);
    for line in file.lines().filter_map(|x| x.ok()) {
        if let Some(c) = Product::new(line) {
            store.add_product(c);
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn load_clients(file: &str, store: &mut Store) -> io::Result<()> {
    let file = BufReader::new(File::open(file)?);
    for line in file.lines().filter_map(|x| x.ok()) {
        if let Some(c) = Client::new(line) {
            store.add_client(c);
        }
    }
    Ok(())
}

#[allow(dead_code)]
pub fn load_sales(file: &str, store: &mut Store) -> io::Result<()> {
    let file = BufReader::new(File::open(file)?);
    for line in file.lines().filter_map(|x| x.ok()) {
        let mut l = line.split_whitespace();
        let product = l.next().unwrap().into();
        let preco = l.next().map(|x| x.parse::<f64>().unwrap()).unwrap();
        let quant = l.next().map(|x| x.parse::<u32>().unwrap()).unwrap();
        let sale = l.next().unwrap() == "P";
        let client = l.next().unwrap().into();
        let mes = l.next().map(|x| x.parse::<u8>().unwrap()).unwrap();
        let filial = l.next().unwrap().into();
        if let Some(s) = Sale::new(product, client, preco, quant, sale, mes, filial) {
            store.add_sale(s);
        }
    }
    Ok(())
}
