#[ic_cdk::query]
fn greet(name: String) -> String {
    format!("Dzień dobry drogi wyborco Rafała Trzaskowskiego, {}! XD", name)
}
