use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

mod report;

use report::{Report, ReportSafety};

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let mut reports: Vec<Report> = Vec::new();

    for line in reader.lines().filter_map(Result::ok) {
        if let Ok(report) = Report::parse(line) {
            reports.push(report)
        }
    }

    let safe_reports_count = reports
        .iter()
        .filter(|report| matches!(report.test_safelty(), ReportSafety::Safe))
        .count();

    let safe_reports_with_problem_dampener_count = reports
        .iter()
        .filter(|report| matches!(report.test_safelty_with_problem_dampener(), ReportSafety::Safe))
        .count();

    println!("Safe reports count: {}", safe_reports_count);
    println!("Safe reports with problem dampener count: {}", safe_reports_with_problem_dampener_count);

    Ok(())
}
