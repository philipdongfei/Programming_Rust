fn main() {
    // Each call to `next_line` returns either `Some(line)`, where
    // `line` is a line of input, or `None`, if we've reached the end of
    // the input. Return the first line that starts with "answer:".
    // Otherwise, return "answer: nothing".
    let answer = loop {
        if let Some(line) = next_line() {
            if line.starts_with("answer: ") {
                break line;
            }
        } else {
            break "answer: nothing";
        }
    };

    // Read some data, one line at a time.
    for line in input_lines {
        let trimmed = trim_comments_and_whitespace(line);
        if trimmed.is_empty() {
            // Jump back to the top of the loop and
            // move on to the next line of input.
            continue;
        }
        // TODO:
    }

    // labeled with a lifetime
    'search:
        for room in apartment {
            for spot in room.hiding_spots() {
                if spot.contains(keys) {
                    println!("Your keys are {} in the {}.", spot, room);
                    break 'search;
                }
        }
        }
    // Find the square root of the first perfect square
    // in the series.
    let sqrt = 'outer: loop {
        let n = next_number();
        for i in 1.. {
            let square = i * i;
            if square == n {
                // Found a square root.
                break 'outer i;
            }
            if square > n {
                // `n` isn't a perfect square, try the next
                break;
            }
        }
    };
}
