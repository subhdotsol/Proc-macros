use macros::ToQuery;
use traits::ToQuery;

#[derive(ToQuery)]
struct Search {
    term: String,
    page: u32,
    #[query_rename = "per_page"]
    per_page: Option<u32>,
}

fn main() {
    let search = Search {
        term: "rust".into(),
        page: 2,
        per_page: Some(50),
    };

    println!("{}", search.to_query());
    // output : term= rust&page=2&per_page = 50
}
