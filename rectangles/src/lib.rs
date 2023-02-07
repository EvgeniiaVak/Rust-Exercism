fn is_vertical(c: char) -> bool {
    c == '|' || c == '+'
}

fn is_horizontal(c: char) -> bool {
    c == '-' || c == '+'
}

pub fn count(lines: &[&str]) -> u32 {
    let mut count = 0;
    if lines.is_empty() || lines[0].is_empty() {
        return count;
    }

    let chars: Vec<Vec<char>> = lines.iter().map(|line| line.chars().collect()).collect();

    for top in 0..chars.len() - 1 {
        for left in 0..chars[top].len() - 1 {
            for right in left + 1..chars[top].len() {
                for bottom in top + 1..chars.len() {
                    if chars[top][left] == '+'
                        && chars[top][right] == '+'
                        && chars[bottom][left] == '+'
                        && chars[bottom][right] == '+'
                    {
                        // check for incomplete rectangle
                        if (left..right).any(|col| {
                            !is_horizontal(chars[top][col]) || !is_horizontal(chars[bottom][col])
                        }) {
                            continue;
                        }
                        if (top..bottom).any(|row| {
                            !is_vertical(chars[row][left]) || !is_vertical(chars[row][right])
                        }) {
                            continue;
                        }

                        // if we get here, we have a complete rectangle
                        count += 1;
                    }
                }
            }
        }
    }

    count
}
