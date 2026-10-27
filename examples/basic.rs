fn main() {
    let result = squery::parse("authentication status:open comments:>10 -is:draft author:");
    for term in &result.query.terms { println!("{term:?}"); }
    for diagnostic in &result.diagnostics { eprintln!("{diagnostic:?}"); }
}
