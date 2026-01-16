use chrono::prelude::*;
use colored::Colorize;

fn main() {
    println!("{}", "Survey Programme Validation".bold());
    let start_date = Utc.with_ymd_and_hms(2026, 3, 6, 12, 0, 0).unwrap();
    let today_date = Utc::now();
    let elapsed = start_date.signed_duration_since(today_date);
    println!(
        "There are about {} weeks until SPV.",
        elapsed.num_weeks().to_string().bold().red()
    );
}
