pub mod client;
pub mod product;
pub mod sale;

use self::client::Client;
use self::product::Product;
use self::sale::{Filial, Sale};
use crate::util::Month;

use std::collections::btree_map::BTreeMap;
use std::sync::RwLock;

#[derive(Debug)]
pub struct TotalBilled {
    n_sales_n: (u32, u32, u32),
    n_sales_p: (u32, u32, u32),
    billed_n: (f64, f64, f64),
    billed_p: (f64, f64, f64),
}

impl Default for TotalBilled {
    fn default() -> Self {
        TotalBilled {
            n_sales_n: (0, 0, 0),
            n_sales_p: (0, 0, 0),
            billed_n: (0.0, 0.0, 0.0),
            billed_p: (0.0, 0.0, 0.0),
        }
    }
}

pub struct Store {
    products: BTreeMap<String, (Product, bool, bool, bool, bool)>,
    clients: BTreeMap<String, Client>,
    sales: [[Vec<Sale>; 12]; 3],
    n_non_bought_products: RwLock<Option<usize>>,
}

impl Store {
    pub fn new() -> Self {
        Store {
            products: BTreeMap::new(),
            clients: BTreeMap::new(),
            sales: [
                [
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ],
                [
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ],
                [
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                    vec![],
                ],
            ],
            n_non_bought_products: RwLock::new(None),
        }
    }

    pub fn add_product(&mut self, product: Product) {
        self.products.insert(
            String::from(product.id()),
            (product, false, false, false, false),
        );
    }

    pub fn add_client(&mut self, client: Client) {
        self.clients.insert(String::from(client.id()), client);
    }

    pub fn add_sale(&mut self, sale: Sale) {
        if self.clients.contains_key(sale.client()) && self.products.contains_key(sale.product()) {
            self.products
                .entry(sale.product().to_string())
                .and_modify(|p| p.1 = true)
                .and_modify(|p| {
                    if !p.2 {
                        p.2 = sale.filial() == Filial::One
                    }
                })
                .and_modify(|p| {
                    if !p.3 {
                        p.3 = sale.filial() == Filial::Two
                    }
                })
                .and_modify(|p| {
                    if !p.4 {
                        p.4 = sale.filial() == Filial::Three
                    }
                });
            self.clients
                .get_mut(sale.client())
                .map(|c| c.make_purchase(sale.filial()));
            self.sales[sale.filial().as_u8() as usize - 1][sale.month().as_u8() as usize - 1]
                .push(sale);
        }
    }

    pub fn serialize(&self) -> std::io::Result<()> {
        use std::fmt::Write as fmtW;
        use std::fs::File;
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
        for v in self
            .sales
            .iter()
            .flat_map(|x| x.iter())
            .flat_map(|x| x.iter())
        {
            writeln!(sv, "{}", v).unwrap();
        }
        let mut f = File::create("db/Vendas_1M._Valid.txt")?;
        f.write_all(sv.as_bytes())?;
        Ok(())
    }

    /// Query 1
    pub fn stats(&self) -> (usize, usize, usize) {
        (
            self.clients.len(),
            self.products.len(),
            self.sales.iter().flatten().map(|v| v.len()).sum(),
        )
    }

    /// Query 2
    pub fn list_by_first_letter(&self, l: char) -> Vec<&Product> {
        let start = format!("{}{}", l, "A0000").to_uppercase();
        let end = format!("{}{}", l, "Z9999").to_uppercase();
        self.products.range(start..end).map(|(_, x)| &x.0).collect()
    }

    /// Query 3
    pub fn total_billed(&self, month: Month, product: &str) -> TotalBilled {
        #[derive(Default, Clone, Copy)]
        struct Billing {
            count: u32,
            price: f64,
        }
        #[derive(Default, Clone, Copy)]
        struct FillialBilling {
            normal: Billing,
            promotion: Billing,
        }
        let billings = self
            .sales
            .iter()
            .map(|filial| {
                filial[(month.as_u8() - 1) as usize]
                    .iter()
                    .filter(|x| x.product() == product)
                    .fold(FillialBilling::default(), |mut fb, s| {
                        if s.promotion() {
                            fb.promotion.count += 1;
                            fb.promotion.price += s.total_price();
                        } else {
                            fb.normal.count += 1;
                            fb.normal.price += s.total_price();
                        };
                        fb
                    })
            })
            .collect::<Vec<_>>();
        let b1 = billings[0];
        let b2 = billings[1];
        let b3 = billings[2];
        TotalBilled {
            n_sales_p: (b1.promotion.count, b2.promotion.count, b3.promotion.count),
            n_sales_n: (b1.normal.count, b2.normal.count, b3.normal.count),
            billed_p: (b1.promotion.price, b2.promotion.price, b3.promotion.price),
            billed_n: (b1.normal.price, b2.normal.price, b3.normal.price),
        }
    }

    /// Query 4
    pub fn never_bought(&self) -> Vec<&Product> {
        let never_bought = self
            .products
            .values()
            .filter(|(_, sold, _, _, _)| !*sold)
            .map(|p| &p.0)
            .collect::<Vec<&Product>>();
        *self.n_non_bought_products.write().unwrap() = Some(never_bought.len());
        never_bought
    }

    /// Query 4
    pub fn never_bought_filial(&self, filial: Filial) -> Vec<&Product> {
        let never_bought = self
            .products
            .values()
            .filter(|p| match filial {
                Filial::One => !p.2,
                Filial::Two => !p.3,
                Filial::Three => !p.4,
            })
            .map(|p| &p.0)
            .collect::<Vec<&Product>>();
        never_bought
    }

    /// Query 5
    pub fn buyers_in_all_filials(&self) -> Vec<&Client> {
        self.clients
            .values()
            .filter(|c| {
                let p = c.purchases();
                p.0 > 0 && p.1 > 0 && p.2 > 0
            })
            .collect()
    }

    /// Query 6
    pub fn n_buyers_without_purchases(&self) -> usize {
        self.clients
            .values()
            .filter(|c| c.purchases() == (0, 0, 0))
            .count()
    }

    /// Query 6
    pub fn n_products_never_bought(&self) -> usize {
        let n_never_bought: Option<usize> = { *self.n_non_bought_products.read().unwrap() };
        match n_never_bought {
            Some(n) => n,
            None => {
                let n = self
                    .products
                    .values()
                    .filter(|(_, sold, _, _, _)| !*sold)
                    .map(|p| &p.0)
                    .count();
                *self.n_non_bought_products.write().unwrap() = Some(n);
                n
            }
        }
    }

    /// Query 7
    pub fn year_purchases(&self, client: &str) -> (Vec<u32>, Vec<u32>, Vec<u32>) {
        let f = |month: &Vec<Sale>| {
            month
                .iter()
                .filter(|s| s.client() == client)
                .map(|s| s.amount())
                .sum()
        };
        let v1 = self.sales[0].iter().map(f).collect::<Vec<u32>>();
        let v2 = self.sales[1].iter().map(f).collect::<Vec<u32>>();
        let v3 = self.sales[2].iter().map(f).collect::<Vec<u32>>();
        (v1, v2, v3)
    }

    /// Query 8
    pub fn total_billed_between(&self, from: Month, to: Month) -> Query8 {
        self.sales.iter().fold(Query8::default(), |acc, filial| {
            filial
                .iter()
                .skip(from.as_u8() as usize - 1)
                .take(to.as_u8() as usize - from.as_u8() as usize + 1)
                .fold(acc, |mut acc, month| {
                    acc.num_items_sold += month // array of Sale of this month
                        .into_iter()
                        .map(Sale::amount)
                        .map(|s| s as usize)
                        .sum::<usize>();
                    acc.num_sales += month.len();
                    acc.total_dolla_dolla += month.into_iter().map(Sale::total_price).sum::<f64>();
                    acc
                })
        })
    }

    /// Query 9
    pub fn product_buyers(&self, product: &str, filial: Filial, promotion: bool) -> Vec<&str> {
        use std::collections::HashSet;
        self.sales[filial.as_u8() as usize - 1]
            .iter()
            .flat_map(|v| v.iter())
            .filter(|s| s.promotion() == promotion)
            .filter(|s| s.product() == product)
            .map(|s| s.client())
            .collect::<HashSet<&str>>()
            .iter()
            .cloned()
            .collect()
    }

    /// Query 10
    pub fn top_purchases(&self, client: &str, month: Month) -> Vec<(&str, u32)> {
        use std::collections::HashMap;
        let mut products: HashMap<&str, u32> = HashMap::new();
        self.sales
            .iter()
            .map(|f| &f[month.as_u8() as usize - 1])
            .flat_map(|x| x.iter())
            .filter(|p| p.client() == client)
            .for_each(|s| {
                if self.products.contains_key(s.product()) {
                    products
                        .entry(s.product())
                        .and_modify(|c| *c += s.amount())
                        .or_insert(s.amount());
                }
            });
        let mut v = products.into_iter().collect::<Vec<(&str, u32)>>();
        v.sort_unstable_by_key(|p| -1 * p.1 as i32);
        v
    }

    /// Query 11
    pub fn top_sold_products(&self, n: usize, sort_by_sales: bool) -> Vec<ProductSales> {
        use std::collections::HashMap;
        let mut table: HashMap<&str, ProductSales> = HashMap::new();
        let mut client_set: HashMap<&str, Vec<&str>> = HashMap::new();
        for s in self.sales.iter().flatten().flatten() {
            match table.get_mut(s.product()) {
                Some(ps) => {
                    let cs = client_set.get_mut(s.product()).unwrap();
                    if !cs.contains(&s.client()) {
                        ps.n_buyers += 1;
                        if s.filial() == Filial::One {
                            ps.n_sold_f1 += s.amount()
                        };
                        if s.filial() == Filial::Two {
                            ps.n_sold_f2 += s.amount()
                        };
                        if s.filial() == Filial::Three {
                            ps.n_sold_f3 += s.amount()
                        };
                        ps.n_sold_total += s.amount();
                        ps.n_sales_f1 += (s.filial() == Filial::One) as u32;
                        ps.n_sales_f2 += (s.filial() == Filial::Two) as u32;
                        ps.n_sales_f3 += (s.filial() == Filial::Three) as u32;
                        ps.n_sales_total += 1;
                        cs.push(s.client());
                    }
                }
                None => {
                    table.insert(
                        s.product(),
                        ProductSales {
                            code: s.product().to_string(),
                            n_buyers: 1,
                            n_sold_f1: if s.filial() == Filial::One {
                                s.amount()
                            } else {
                                0
                            },
                            n_sold_f2: if s.filial() == Filial::Two {
                                s.amount()
                            } else {
                                0
                            },
                            n_sold_f3: if s.filial() == Filial::Three {
                                s.amount()
                            } else {
                                0
                            },
                            n_sold_total: s.amount(),
                            n_sales_f1: (s.filial() == Filial::One) as u32,
                            n_sales_f2: (s.filial() == Filial::Two) as u32,
                            n_sales_f3: (s.filial() == Filial::Three) as u32,
                            n_sales_total: 1,
                        },
                    );
                    client_set.insert(s.product(), vec![s.client()]);
                }
            };
        }
        let mut pss = table.into_iter().map(|ps| ps.1).collect::<Vec<_>>();
        if sort_by_sales {
            pss.sort_unstable_by_key(|ps| -1 * ps.n_sales_total as i32);
        } else {
            pss.sort_unstable_by_key(|ps| -1 * ps.n_sold_total as i32);
        }
        pss.iter().take(n).cloned().collect()
    }

    /// Query 12
    pub fn top_expense(&self, client: &str, n: usize) -> Vec<Expense> {
        use std::collections::{BinaryHeap, HashMap};
        let mut products: HashMap<&str, f64> = HashMap::new();
        self.sales
            .iter()
            .flat_map(|x| x.iter())
            .flat_map(|x| x.iter())
            .filter(|s| s.client() == client)
            .for_each(|s| {
                products
                    .entry(s.product())
                    .and_modify(|c| *c += s.total_price())
                    .or_insert(s.total_price());
            });
        let mut heap = products
            .into_iter()
            .map(|(prod, total)| Expense(total, prod))
            .collect::<BinaryHeap<_>>();
        let mut ps = Vec::with_capacity(n);
        while let Some(p) = heap.pop() {
            if ps.len() == n {
                break;
            }
            ps.push(p);
        }
        ps
    }
}

#[derive(Default, Debug)]
pub struct Query8 {
    num_sales: usize,
    num_items_sold: usize,
    total_dolla_dolla: f64,
}

// N mais vendidos
#[derive(Debug, Clone)]
pub struct ProductSales {
    code: String,
    n_buyers: usize,
    n_sold_f1: u32,
    n_sold_f2: u32,
    n_sold_f3: u32,
    n_sold_total: u32,
    n_sales_f1: u32,
    n_sales_f2: u32,
    n_sales_f3: u32,
    n_sales_total: u32,
}

#[derive(PartialEq, PartialOrd)]
pub struct Expense<'a>(f64, &'a str);

impl<'a> Eq for Expense<'a> {}

impl<'a> Ord for Expense<'a> {
    fn cmp(&self, other: &Expense<'a>) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> std::fmt::Display for Expense<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{} : {}", self.1, self.0)
    }
}
