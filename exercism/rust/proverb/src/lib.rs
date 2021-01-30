pub fn build_proverb(list: &[&str]) -> String {
    if list.len() == 0 {
        return String::new();
    }
    let mut line: Vec<String> = list
        .windows(2)
        .map(|win| format!("For want of a {} the {} was lost.", win[0], win[1]))
        .collect();
    line.push(format!("And all for the want of a {}.", list[0]));
    line.join("\n")
}
