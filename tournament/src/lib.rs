use std::cmp::Ordering;
use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {
    let mut rows = parse_results(match_results);
    rows.sort();
    format_table(rows)
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    team: String,
    matches_played: u32,
    wins: u32,
    draws: u32,
    losses: u32,
    points: u32,
}

impl PartialOrd for Row {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.points == other.points {
            return self.team.partial_cmp(&other.team);
        }

        // inverse ordering by points
        // the larger the points, the higher the rank
        other.points.partial_cmp(&self.points)
    }
}

impl Ord for Row {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

fn inverse_result(result: &str) -> &str {
    match result {
        "win" => "loss",
        "loss" => "win",
        "draw" => "draw",
        _ => panic!("Invalid result: {result}"),
    }
}

fn update_row(row: &mut Row, result: &str) {
    row.matches_played += 1;
    match result {
        "win" => {
            row.wins += 1;
            row.points += 3;
        }
        "loss" => {
            row.losses += 1;
        }
        "draw" => {
            row.draws += 1;
            row.points += 1;
        }
        _ => panic!("Invalid result: {result}"),
    };
}

fn parse_results(match_results: &str) -> Vec<Row> {
    let mut stats = HashMap::new();
    for result in match_results.split('\n') {
        if result.is_empty() {
            continue;
        }
        let mut result = result.split(';');
        let team_a = result.next().unwrap();
        let team_b = result.next().unwrap();
        let result = result.next().unwrap();

        let fresh_row = |team: &str| Row {
            team: team.to_string(),
            matches_played: 0,
            wins: 0,
            draws: 0,
            losses: 0,
            points: 0,
        };

        let team_a = stats.entry(team_a).or_insert_with(|| fresh_row(team_a));
        update_row(team_a, result);

        let team_b = stats.entry(team_b).or_insert_with(|| fresh_row(team_b));
        update_row(team_b, inverse_result(result));
    }

    stats.values().cloned().collect()
}

fn format_table(rows: Vec<Row>) -> String {
    let mut result = String::new();
    result.push_str("Team                           | MP |  W |  D |  L |  P\n");
    for row in rows {
        result.push_str(&format!(
            "{:31}| {:2} | {:2} | {:2} | {:2} | {:2}\n",
            row.team, row.matches_played, row.wins, row.draws, row.losses, row.points
        ));
    }
    result.trim().to_string()
}

#[cfg(test)]
mod tests {
    #[test]
    fn format_table() {
        let table = vec![
            super::Row {
                team: "Allegoric Alaskans".to_string(),
                matches_played: 2,
                wins: 2,
                draws: 0,
                losses: 0,
                points: 6,
            },
            super::Row {
                team: "Blithering Badgers".to_string(),
                matches_played: 2,
                wins: 0,
                draws: 0,
                losses: 2,
                points: 0,
            },
        ];
        let expected = "".to_string()
            + "Team                           | MP |  W |  D |  L |  P\n"
            + "Allegoric Alaskans             |  2 |  2 |  0 |  0 |  6\n"
            + "Blithering Badgers             |  2 |  0 |  0 |  2 |  0";
        assert_eq!(super::format_table(table), expected);
    }
}
