use crate::store::{ TotalBilled, ProductSales, Expense };
use crate::store::product::Product;
use crate::store::client::Client;

// Query 2
pub fn list_by_first_letter(prods: Vec<&Product>) -> String {
    let mut response = String::new();
    let total = prods.len();
    for p in prods {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}", total);
    response
}

// Query 3
pub fn total_billed(total_billed: TotalBilled) -> String {
    format!("{:#?}", total_billed)
}

// Query 4
pub fn never_bought(prods: Vec<&Product>) -> String {
    let mut response = String::new();
    for p in prods.iter() {
        response += &format!("{}\n", p);
    }
    response += &format!("TOTAL: {}\n", prods.len());
    response
}

// Query 5
pub fn buyers_in_all_filials(clients: Vec<&Client>) -> String {
    let mut response = String::new();
    for p in clients.iter() {
        response += &format!("{}\n", p);
    }
    response
}

// Query 6
pub fn never_bought_never_purchased(n_buyers: usize, n_products: usize) -> String {
    format!("clients: {} | products: {}", n_buyers, n_products)
}

// Query 7
pub fn year_purchases(table: (Vec<u32>, Vec<u32>, Vec<u32>)) -> String {
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

// Query 8
pub fn total_billed_between(billed: (usize, f64)) -> String {
    format!("{:?}", billed)
}

// Query 9
pub fn product_buyers(clients: Vec<&str>) -> String {
    let mut response = String::new();
    for p in clients {
        response += &format!("{}\n", p);
    }
    response
}

// Query 10
pub fn top_purchases(prods: Vec<(&Product, u32)>) -> String {
    let mut response = String::new();
    for p in prods {
        response += &format!("{} : {}", p.0, p.1);
    }
    response
}

// Query 11
pub fn top_sold_products(prods: Vec<ProductSales>) -> String {
    let mut response = String::new();
    for p in prods {
        response += &format!("{:?}\n", p);
    }
    response
}

// Query 12
pub fn top_expense(top_expenses: (Option<Expense>, Option<Expense>, Option<Expense>)) -> String {
    let mut response = String::new();
    if let Some(p) = top_expenses.0 { response += &format!("{}\n", p); }
    if let Some(p) = top_expenses.1 { response += &format!("{}\n", p); }
    if let Some(p) = top_expenses.2 { response += &format!("{}\n", p); }
    response
}

