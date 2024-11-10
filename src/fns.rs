use colored::Colorize;
use rand::Rng;
use std::io;

pub fn get_cols() -> u16 {
    termsize::get()
        .unwrap_or(termsize::Size { rows: 64, cols: 64 })
        .cols
}

pub fn br() {
    println!(
        "\n{}\n",
        std::iter::repeat('-')
            .take(get_cols().into())
            .collect::<String>()
    )
}

pub fn print_space() {
    const SPACEY_THINGS: &[char] = &[
        '⟡', '₊', '˚', '⊹', '⟡', '₊', '˚', '⊹', '⟡', '₊', '˚', '⊹', '⟡', '₊', '˚', '⊹', '⟡', '₊',
        '˚', '⊹', '⟡', '₊', '˚', '⊹', '⟡', '₊', '˚', '⊹', '⟡', '₊', '˚', '⊹', '👽', '👾', '🛰',
        '🚀', '🛸', '🌕', '☀', '🪐', '🌌', '☄', '·', '·', '·', '·', '·',
    ];

    let mut rng = rand::thread_rng();
    // let mut line = std::iter::repeat(' ')
    //     .take(get_cols().into())
    //     .collect::<String>();

    let line = (0..get_cols())
        .into_iter()
        .map(|_|
            // WHYY
            {
                if rng.gen_bool(0.1) {
                    SPACEY_THINGS[rng.gen_range(0..SPACEY_THINGS.len())]
                } else {
                    ' '
                }
            })
        .collect::<String>();
    println!("{}", line.get(0..get_cols().into()).unwrap_or_default());
}

pub fn get_y_n() -> bool {
    loop {
        let mut buf = String::new();
        io::stdin().read_line(&mut buf).expect("Couldn't get input");
        let inp = buf.trim().to_ascii_lowercase();
        if inp.len() == 1
            && ["y", "n"].contains(
                &inp.chars()
                    .nth(0)
                    .expect("No 1st char")
                    .to_string()
                    .as_str(),
            )
        {
            break inp == "y".to_string();
        }
    }
}

pub fn wait() {
    io::stdin().read_line(&mut String::new()).unwrap();
}

pub fn cls() {
    clearscreen::clear().expect("failed to clear screen");
}

pub fn colour_crew_string(p: u32) -> String {
    match p {
        800..=1000 => format!("{p}").green().to_string(),
        600..800 => format!("{p}").bright_green().to_string(),
        400..600 => format!("{p}").yellow().to_string(),
        200..400 => format!("{p}").bright_red().to_string(),
        0..200 => format!("{p}").red().to_string(),
        _ => format!("{p}"),
    }
}
