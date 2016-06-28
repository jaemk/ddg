extern crate ddg;

use ddg::Query;

fn main() {
    let do_cli = unimplemented!();

    let search = Query::new("rust").format("xml").callback("parseResponse").pretty();

    let result = search.execute();

    //    println!("{}", result.results);
}
