pub mod client;
pub mod product;
pub mod sale;

use self::product::Product;
use self::client::Client;
use self::sale::Sale;
use crate::util::Month;

use std::collections::btree_map::BTreeMap;
use std::cell::Cell;

#[derive(Debug)]
pub struct TotalBilled {
    n_sales_n: (u32, u32, u32),
    n_sales_p: (u32, u32, u32),
    billed_n: (f64, f64, f64),
    billed_p: (f64, f64, f64),
}

impl Default for TotalBilled {
    fn default() -> Self {
        TotalBilled{
            n_sales_n: (0,0,0),
            n_sales_p: (0,0,0),
            billed_n: (0.0,0.0,0.0),
            billed_p: (0.0,0.0,0.0)
        }
    }
}

pub struct Store {
    products: BTreeMap<String, (Product, bool)>,
    clients: BTreeMap<String, Client>,
    sales: [[Vec<Sale>; 12]; 3],
    n_non_bought_products: Cell<Option<usize>>
}

impl Store {
    pub fn new() -> Self {
        Store {
            products: BTreeMap::new(),
            clients: BTreeMap::new(),
            sales: [
                [vec![], vec![],vec![], vec![], vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![]],
                [vec![], vec![],vec![], vec![], vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![]],
                [vec![], vec![],vec![], vec![], vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![]],
            ],
            n_non_bought_products: Cell::new(None),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(String::from(product.id()), (product, false));
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.insert(String::from(client.id()), client);
    }

    pub fn add_sale(&mut self, sale: Sale) {
        if self.clients.contains_key(sale.client())
            && self.products.contains_key(sale.product())
            {
                self.products.entry(sale.product().to_string())
                    .and_modify(|(_, b)| *b = true);
                self.clients.get_mut(sale.client()).map(|c| c.make_purchase(sale.filial()));
                self.sales[sale.filial().as_u8() as usize - 1][sale.month().as_u8() as usize - 1].push(sale);
            }
    }

    pub fn serialize(&self) -> std::io::Result<()> {
        use std::fs::File;
        use std::fmt::Write as fmtW;
        use std::io::Write as ioW;
        let mut cv = String::new();
        for c in self.clients.values() {
            writeln!(cv, "{}", c).unwrap();
        }
        let mut f = File::create("db/Clientes_Valid.txt")?;
        f.write_all(cv.as_bytes())?;
        let mut pv = String::new();
        for p in self.products.values() {
            writeln!(pv, "{}", p.0).unwrap();
        }
        let mut f = File::create("db/Produtos_Valid.txt")?;
        f.write_all(pv.as_bytes())?;
        let mut sv = String::new();
        for v in self.sales.iter().flat_map(|x| x.iter()).flat_map(|x| x.iter()) {
            writeln!(sv, "{}", v).unwrap();
        }
        let mut f = File::create("db/Vendas_1M._Valid.txt")?;
        f.write_all(sv.as_bytes())?;
        Ok(())
    }

    pub fn list_by_first_letter(&self, l: char) -> Vec<&Product> {
        let start = format!("{}{}", l, "A0000").to_uppercase();
        let end = format!("{}{}", l, "Z9999").to_uppercase();
        self.products.range(start..end).map(|(_,x)| &x.0).collect()
    }

    pub fn total_billed(&self, month: Month, product: String) -> TotalBilled {
        let billings = self.sales.iter().map(|filial| {
            filial[(month.as_u8() - 1) as usize].iter()
                .filter(|x| x.product() == &product)
                .fold(((0,0.0),(0,0.0)), |(mut n, mut p), s| {
                    if s.promotion() {
                        p.0 += 1; p.1 += s.total_price();
                    } else {
                        n.0 += 1; n.1 += s.total_price();
                    };
                    (n, p)
                })
        }).collect::<Vec<_>>();
        let b1 = billings[0];
        let b2 = billings[1];
        let b3 = billings[2];
        TotalBilled {
            n_sales_n: ((b1.1).0, (b2.1).0, (b3.1).0),
            n_sales_p: ((b1.0).0, (b2.0).0, (b3.0).0),
            billed_n: ((b1.1).1, (b2.1).1, (b3.1).1),
            billed_p: ((b1.0).1, (b2.0).1, (b3.0).1),
        }
    }

    pub fn never_bought(&self) -> (usize, Vec<&Product>) {
        let never_bought = self.products
            .values()
            .filter(|(_, sold)| !*sold)
            .map(|p| &p.0)
            .collect::<Vec<&Product>>();
        self.n_non_bought_products.set(Some(never_bought.len()));
        (never_bought.len(), never_bought)
    }

    pub fn buyers_in_all_filials(&self) -> Vec<&Client> {
        self.clients.values()
            .filter(|c| { let p = c.purchases(); p.0 > 0 && p.1 > 0 && p.2 > 0 })
            .collect()
    }

    pub fn n_buyers_without_purchases(&self) -> usize {
        self.clients.values()
            .filter(|c| c.purchases() == (0,0,0))
            .count()
    }

    pub fn n_never_bought(&self) -> usize {
        match self.n_non_bought_products.get() {
            Some(n) => n,
            None => {
                let n = self.products
                    .values()
                    .filter(|(_, sold)| !*sold)
                    .map(|p| &p.0)
                    .count();
                self.n_non_bought_products.set(Some(n));
                n
            }
        }
    }

    pub fn year_purchases(&self, client: String) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let f = |month :&Vec<Sale>| month.iter().filter(|s| s.client() == client).map(|s| s.amount()).sum();
        let v1 = self.sales[0]
            .iter()
            .map(f)
            .collect::<Vec<u32>>();
        let v2 = self.sales[1]
            .iter()
            .map(f)
            .collect::<Vec<u32>>();
        let v3 = self.sales[2]
            .iter()
            .map(f)
            .collect::<Vec<u32>>();
        (v1, v2, v3)
    }

    pub fn total_billed_between(&self, from: Month, to: Month) -> (usize, f64) {
        let mut n_sales = 0;
        let total_sales = self.sales
            .iter()
            .map(|filial| filial.iter()
                 .skip(from.as_u8() as usize - 1)
                 .take(to.as_u8() as usize - 1)
                 .map(|month| {
                     n_sales += month.len();
                     month.iter().fold(0.0, |acc, s| s.total_price() + acc)
                 })
                 .fold(0.0, |s, acc| s + acc)
                )
            .sum();
        (n_sales, total_sales)
    }
}
