#[allow(dead_code)]

fn day1(s: String) -> i32 {
    let board: Vec<Vec<char>> = s
        .as_str()
        .split('\n')
        .map(|x| x.chars().collect())
        .collect();
    let dirs = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
        (1, 0),
        (1, -1),
        (0, -1)
    ];
    let mut count = 0;
    for i in 0..board.len() {
        for j in 0..board[i].len() {
            if board[i][j] == 'X' {
                count += dirs.iter().fold(0, |acc, dir| 
                    acc + if search(
                        &board, 
                        "XMAS".chars().collect(), 
                        1, 
                        (i.try_into().unwrap(), j.try_into().unwrap()), 
                        *dir
                    ) {
                        1
                    } else {
                        0
                    }
                );
            }
        }
    }
    count
}

fn search(
        grid: &Vec<Vec<char>>, 
        word: Vec<char>, 
        index: usize, 
        coord: (i32, i32), 
        dir: (i32, i32)
    ) -> bool {
    if index == word.len() { return true; }
    let new_coord = (coord.0 + dir.0, coord.1 + dir.1);
    match get_letter(grid, coord) {
        Some(c) if c == word[index] => 
            search(grid, word, index+1, new_coord, dir),
        _ => false,
    }
}

fn get_letter(grid: &Vec<Vec<char>>, coord: (i32, i32)) -> Option<char> {
    let x: Option<usize> = coord.0.try_into().ok();
    let y: Option<usize> = coord.1.try_into().ok();
    match (x, y) { // see if coords are positive
        (Some(a), Some(b)) => match grid.get(a) { // check x coord in bounds
            Some(row) => match row.get(b) { // check y coord in bounds
                Some(c) => Some(*c),
                _ => None,
            },
            _ => None,
        },
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d4_small_test() {
        let result = day1(String::from(
            "..X...
            .SAMX.
            .A..A.
            XMAS.S
            .X....")
        );
        assert_eq!(result, 4);
    }
    
}
