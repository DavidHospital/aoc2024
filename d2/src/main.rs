use utils::read_input;

fn main() {
    let raw_lines = read_input("inputs/d2.txt").unwrap();

    let reports: Vec<_> = raw_lines
        .into_iter()
        .map(|line| {
            Report(
                line.split_whitespace()
                    .map(|level| level.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>(),
            )
        })
        .collect();

    // Part A
    let safe_reports: Vec<_> = reports.iter().filter(|report| report.is_safe()).collect();

    println!("Part A: {}", safe_reports.len());

    // Part B
    let safe_reports: Vec<_> = reports
        .iter()
        .filter(|report| report.sub_reports().into_iter().any(|r| r.is_safe()))
        .collect();
    println!("Part B: {}", safe_reports.len());
}

struct Report(Vec<i32>);

impl Report {
    fn is_safe(&self) -> bool {
        let report = &self.0;
        if report.len() <= 1 {
            return true;
        }
        let increasing = report[1] > report[0];
        for idx in 1..report.len() {
            let mut diff = report[idx] - report[idx - 1];
            if !increasing {
                diff *= -1;
            }

            if diff < 1 || diff > 3 {
                return false;
            }
        }
        true
    }

    fn sub_reports(&self) -> Vec<Report> {
        (0..self.0.len())
            .map(|idx| {
                let mut report = self.0.clone();
                report.remove(idx);
                Report(report)
            })
            .collect()
    }
}
