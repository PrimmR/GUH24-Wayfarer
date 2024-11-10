/// Documentation who??
mod fns;

use crate::fns::*;

use colored::Colorize;
use rand::Rng;
use strum::{EnumIter, IntoEnumIterator};

#[derive(Debug, Clone, Copy, strum::Display, EnumIter)]
enum PlanetSize {
    Small,
    Medium,
    Large,
    Massive,
}

#[derive(Debug)]
struct DetectorState {
    val: u8,
    power: u8,
}

impl DetectorState {
    fn new() -> Self {
        Self { val: 100, power: 0 }
    }

    fn pct_string(&self) -> String {
        format_percentage(self.val)
    }

    fn power_string(&self) -> String {
        let on: String = std::iter::repeat('▢').take((self.power).into()).collect();
        let off: String = std::iter::repeat('▢')
            .take((3 - self.power).into())
            .collect();

        format!("{}{}", on.green(), off.white())
    }
}

struct Spaceship {
    detectors: Detectors,
}

impl Spaceship {
    fn new() -> Self {
        Self {
            detectors: Detectors {
                oxygen: DetectorState::new(),
                water: DetectorState::new(),
                energy: DetectorState::new(),
                nutrients: DetectorState::new(),
            },
        }
    }

    fn display(&self) {
        println!(
            r#"Detectors:
-   Oxygen {}:    {}
-   Water {}:     {}
-   Energy {}:    {}
-   Nutrients {}: {}
"#,
            self.detectors.oxygen.pct_string(),
            self.detectors.oxygen.power_string(),
            self.detectors.water.pct_string(),
            self.detectors.water.power_string(),
            self.detectors.energy.pct_string(),
            self.detectors.energy.power_string(),
            self.detectors.nutrients.pct_string(),
            self.detectors.nutrients.power_string(),
        );
    }
}

fn format_percentage(pct: u8) -> String {
    match pct {
        80..=100 => format!("{pct}%").green().to_string(),
        60..80 => format!("{pct}%").bright_green().to_string(),
        40..60 => format!("{pct}%").yellow().to_string(),
        20..40 => format!("{pct}%").bright_red().to_string(),
        0..20 => format!("{pct}%").red().to_string(),
        _ => format!("{pct}%"),
    }
    .bold()
    .to_string()
}

fn format_percentage_w_fail(pct: Option<u8>) -> String {
    match pct {
        Some(v) => format_percentage(v),
        None => String::from("Failed").bold().red().to_string(),
    }
}

struct Detectors {
    oxygen: DetectorState,
    water: DetectorState,
    energy: DetectorState,
    nutrients: DetectorState,
}

struct Planet {
    oxygen: Option<u8>,
    water: Option<u8>,
    energy: Option<u8>,
    nutrients: Option<u8>,
    size: PlanetSize,
}

impl Planet {
    fn new(s: &Spaceship) -> Self {
        let mut rng = rand::thread_rng();

        Self {
            oxygen: rng
                .gen_bool(s.detectors.oxygen.val as f64 / 100.)
                .then_some(rng.gen_range(s.detectors.oxygen.power * 20..=100)),
            water: rng
                .gen_bool(s.detectors.water.val as f64 / 100.)
                .then_some(rng.gen_range(s.detectors.water.power * 20..=100)),
            energy: rng
                .gen_bool(s.detectors.energy.val as f64 / 100.)
                .then_some(rng.gen_range(s.detectors.energy.power * 20..=100)),
            nutrients: rng
                .gen_bool(s.detectors.nutrients.val as f64 / 100.)
                .then_some(rng.gen_range(s.detectors.nutrients.power * 20..=100)),
            size: PlanetSize::iter()
                .nth(rng.gen_range(0..PlanetSize::iter().len()))
                .unwrap(),
        }
    }

    fn display(&self) {
        println!(
            r#"
-   Oxygen: {}
-   Water: {}
-   Energy: {}
-   Nutrients: {}

-   Size: {}"#,
            format_percentage_w_fail(self.oxygen),
            format_percentage_w_fail(self.water),
            format_percentage_w_fail(self.energy),
            format_percentage_w_fail(self.nutrients),
            self.size
        );
    }

    fn score(&self) -> f32 {
        let mut rng = rand::thread_rng();

        let l = self.oxygen.unwrap_or(rng.gen_range(0..=100));
        let w = self.water.unwrap_or(rng.gen_range(0..=100));
        let e = self.energy.unwrap_or(rng.gen_range(0..=100));
        let n = self.nutrients.unwrap_or(rng.gen_range(0..=100));

        Planet {
            oxygen: Some(l),
            water: Some(w),
            energy: Some(e),
            nutrients: Some(n),
            size: self.size,
        }
        .display();

        let l = (l as f32) / 100.;
        let w = (w as f32) / 100.;
        let e = (e as f32) / 100.;
        let n = (n as f32) / 100.;

        let s = match self.size {
            PlanetSize::Small => 40,
            PlanetSize::Medium => 100,
            PlanetSize::Large => 80,
            PlanetSize::Massive => 20,
        } as f32;

        let score = l * w * e * n * s * 10000.;
        score.round()
    }
}

fn event(s: &mut Spaceship) {
    let mut rng = rand::thread_rng();

    // Good / Bad
    if rng.gen_bool(0.5) {
        // Bad (collision)
        let hit = rng.gen_range(0..4);
        println!("Oh no, your spaceship has crashed into an asteroid!!");
        let s = match hit {
            0 => {
                s.detectors.oxygen.val =
                    s.detectors.oxygen.val.saturating_sub(rng.gen_range(5..25));
                println!("Your oxygen sensors were hit!");
                s.detectors.oxygen.pct_string()
            }
            1 => {
                s.detectors.water.val = s.detectors.water.val.saturating_sub(rng.gen_range(5..25));

                println!("Your water sensors were hit!");
                s.detectors.water.pct_string()
            }
            2 => {
                s.detectors.energy.val =
                    s.detectors.energy.val.saturating_sub(rng.gen_range(5..25));

                println!("Your energy sensors were hit!");
                s.detectors.energy.pct_string()
            }
            3 => {
                s.detectors.nutrients.val = s
                    .detectors
                    .nutrients
                    .val
                    .saturating_sub(rng.gen_range(5..25));

                println!("Your nutrients sensors were hit!");
                s.detectors.nutrients.pct_string()
            }
            _ => panic!(),
        };
        println!("They are now operating at {} capactity", s);
    } else {
        // Good (knowledge)
        println!("Your spaceship was doing some data processing and acquired some new knowledge.");

        if s.detectors.oxygen.power >= 3
            && s.detectors.water.power >= 3
            && s.detectors.energy.power >= 3
            && s.detectors.nutrients.power >= 3
        {
            println!("But your spaceship is already omniscient!");
            return;
        }

        println!(
            r#"Detectors:
-   Oxygen {}
-   Water {}
-   Energy {}
-   Nutrients {}
"#,
            s.detectors.oxygen.power_string(),
            s.detectors.water.power_string(),
            s.detectors.energy.power_string(),
            s.detectors.nutrients.power_string(),
        );

        println!(
            "Which sensor would you like to upgrade? (1) oxygen (2) water (3) energy (4) nutrients"
        );
        loop {
            let mut inp = String::new();
            std::io::stdin().read_line(&mut inp).unwrap();

            if let Ok(v) = inp.trim().parse::<u8>() {
                match v {
                    1 => {
                        if s.detectors.oxygen.power <= 3 {
                            s.detectors.oxygen.power += 1;
                            println!("You upgraded the oxygen sensor - you will now find planets with more abundant oxygen resources");
                            break;
                        } else {
                            println!("You have already upgraded the oxygen sensor to 100%")
                        }
                    }
                    2 => {
                        if s.detectors.water.power <= 3 {
                            s.detectors.water.power += 1;
                            println!("You upgraded the water sensor - you will now find planets with more abundant water resources");
                            break;
                        } else {
                            println!("You have already upgraded the oxygen sensor to 100%")
                        }
                    }
                    3 => {
                        if s.detectors.energy.power <= 3 {
                            s.detectors.energy.power += 1;
                            println!("You upgraded the power sensor - you will now find planets with more abundant power resources");
                            break;
                        } else {
                            println!("You have already upgraded the oxygen sensor to 100%")
                        }
                    }
                    4 => {
                        if s.detectors.nutrients.power < 3 {
                            s.detectors.nutrients.power += 1;
                            println!("You upgraded the nutrients sensor - you will now find planets with more abundant nutrients resources");
                            break;
                        } else {
                            println!("You have already upgraded the oxygen sensor to 100%")
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut s = Spaceship::new();
    let mut year: u32 = 2024;

    loop {
        cls();

        br();

        println!(
            "Welcome to the year {}\nYour spaceship has identified a new planet",
            format!("{}", year).bold().blue()
        );

        s.display();

        year += rng.gen_range(200..=1000);
        let p = Planet::new(&s);

        println!("\n\n");

        for _ in 0..5 {
            print_space();
        }
        println!("\n\n");

        println!("New planet detected:\n");

        p.display();

        br();

        println!("Would you like to continue searching (Y/N)");
        if !get_y_n() {
            println!("Name your planet");
            let mut name = String::new();
            std::io::stdin().read_line(&mut name).unwrap();

            br();

            println!("You landed on your lovely new home of: {}", name);
            println!();
            println!(
                "Your score was: {}, not bad",
                p.score().to_string().bold()
            );
            wait();

            br();

            break;
        }

        cls();

        br();

        event(&mut s);

        br();

        wait();

        cls();
    }
}
