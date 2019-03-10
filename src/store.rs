pub mod client;
pub mod product;
pub mod sale;

use self::product::Product;
use self::client::Client;
use self::sale::Sale;
use crate::util::Month;

use std::collections::hash_map::HashMap;
use std::collections::btree_map::BTreeMap;

pub struct Store {
    products: BTreeMap<String, (Product, bool)>,
    clients: HashMap<String, Client>,
    sales: [Vec<Sale>; 12],
}

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

impl Store {
    pub fn new() -> Self {
        Store {
            products: BTreeMap::new(),
            clients: HashMap::new(),
            sales: [vec![], vec![],vec![], vec![], vec![], vec![],vec![], vec![],vec![], vec![],vec![], vec![]],
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
                self.sales[sale.month() as usize - 1].push(sale);
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
        for v in self.sales.iter().flat_map(|x| x.iter()) {
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
        use self::sale::Filial;
        let f = |sales :&mut (u32, u32 ,u32), bills :&mut (f64, f64, f64), s :&Sale| {
            match s.filial() {
                Filial::One => { sales.0 += 1; bills.0 += s.price() * s.amount() as f64 },
                Filial::Two => { sales.1 += 1; bills.1 += s.price() * s.amount() as f64 },
                Filial::Three => { sales.2 += 1; bills.2 += s.price() * s.amount() as f64 },
                _ => (),
            };
        };
        self.sales[(month.as_u8() - 1) as usize].iter()
            .filter(|x| x.product() == &product)
            .fold(TotalBilled::default(), |mut bills, s| {
                if s.promotion() {
                    f(&mut bills.n_sales_p, &mut bills.billed_p, s);
                } else {
                    f(&mut bills.n_sales_n, &mut bills.billed_n, s);
                };
                bills
            })
    }

    pub fn never_bought(&self) -> (usize, Vec<&Product>) {
        let never_bought = self.products
            .values()
            .filter(|(_, sold)| !*sold)
            .map(|p| &p.0)
            .collect::<Vec<&Product>>();
        (never_bought.len(), never_bought)
    }
}
