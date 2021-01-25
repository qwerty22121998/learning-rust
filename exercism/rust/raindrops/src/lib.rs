pub fn raindrops(n: u32) -> String {
    let res: String = [(3, "Pling"), (5, "Plang"), (7, "Plong")]
        .iter()
        .filter(|&&(e, _)| n % e == 0)
        .map(|&(_, e)| e)
        .collect();
    if res.is_empty() {
        n.to_string()
    } else {
        res
    }
}
