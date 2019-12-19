fn subverse(n: u32) -> String {
    match n {
        0 => format!("no more bottles"),
        1 => format!("1 bottle"),
        _ => format!("{} bottles", n),
    }
}

fn get_pronoum(n: u32) -> String {
    match n {
        1 => format!("it"),
        _ => format!("one")
    }
}

fn get_first_part(n: u32) -> String {
    match n {
        0 => format!("No more bottles of beer on the wall, no more bottles of beer."),
        _ => format!(
            "{} of beer on the wall, {} of beer.",
            subverse(n),
            subverse(n)
        ),
    }
}

fn get_second_part(n: u32) -> String {
    match n {
        0 => format!("Go to the store and buy some more, 99 bottles of beer on the wall."),
        _ => format!(
            "Take {} down and pass it around, {} of beer on the wall.",
            get_pronoum(n),
            subverse(n - 1)
        ),
    }
}

pub fn verse(n: u32) -> String {
    format!("{}\n{}\n", get_first_part(n), get_second_part(n))
}

pub fn sing(from: u32, to: u32) -> String {
    (to .. from + 1).rev()
                    .map(verse)
                    .collect::<Vec<_>>()
                    .join("\n")
}
