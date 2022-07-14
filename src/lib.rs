use array2d::Array2D;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Entry {
    Num(u32),
    Empty([bool; 9]), //Which values are possible
}

type Line = [Entry; 9]; // [Line] = [[Entry]] line is a horizontal line and every entry is a number in the sudoku

pub fn run(contents: &str) -> Result<String, String> {
    let mut parsed = parse_contents(contents)?;
    
    solve(&mut parsed)?;

    Ok(unparse_sudoku(parsed))
}

fn parse_contents(contents: &str) -> Result<Array2D<Entry>, String> {
    const RADIX: u32 = 10;

    let mut vec: Vec<Vec<Entry>> = vec![];

    //Fill Vec<Line>
    for line in contents.lines() {
        if line.len() != 9 {
            return Err(format!("Expected a 9x9 sudoku but got a line length of '{}' instead!", line.len()));
        }
        let mut line_vec = vec![];
        for character in line.chars() {
            line_vec.push(
                match character.to_digit(RADIX) {
                Some(num) => Entry::Num(num),
                None => Entry::Empty([true; 9]),
            });
        };
        vec.push(line_vec);
    };

    let array = Array2D::from_rows(&vec);
    Ok(array)
}

fn solve(sudoku: &mut Array2D<Entry>) -> Result<(), String> {
    let mut iterations = 0;
    // Loop while sudoku contains empty entries i.e. is unsolved
    while sudoku.elements_column_major_iter()
                .any(|entry| if let Entry::Empty(_) = entry { true } else { false } ) {
        
        for line_nr in 0..9 {
            for col_nr in 0..9 {
                analyze(line_nr, col_nr, sudoku);
            }
        }

        update_sudoku(sudoku);

        if iterations >= 20 {
            return Err(String::from("Could not find solution!"));
        }

        iterations += 1;
    }

    Ok(())
}

fn analyze(line_nr: usize, col_nr: usize, sudoku: &mut Array2D<Entry>) {
    let sudoku_clone = sudoku.clone();

    let line = sudoku.as_rows()[line_nr];
    let column = sudoku.as_columns()[col_nr];

    if let Entry::Empty(ref mut inner_array) = sudoku[(line_nr, col_nr)] {

        //Check line
        for entry in line {
            if let Entry::Num(num) = entry {
                inner_array[num as usize - 1] = false;
            }
        }

        //Check column
        for entry in column {
            if let Entry::Num(num) = entry {
                inner_array[num as usize - 1] = false;
            }
        }

    //One cell is one of the 9 3x3 grids in the sudoku
    let mut cells: Vec<Vec<Entry>> = vec![Vec::with_capacity(9); 9];

    for i in 0..3 {
        cells[0].extend_from_slice(&sudoku_clone[i][0..3]);
        cells[1].extend_from_slice(&sudoku_clone[i][3..6]);
        cells[2].extend_from_slice(&sudoku_clone[i][6..9]);
    }

    for i in 3..6 {
        cells[3].extend_from_slice(&sudoku_clone[i][0..3]);
        cells[4].extend_from_slice(&sudoku_clone[i][3..6]);
        cells[5].extend_from_slice(&sudoku_clone[i][6..9]);
    }
    
    for i in 6..9 {
        cells[6].extend_from_slice(&sudoku_clone[i][0..3]);
        cells[7].extend_from_slice(&sudoku_clone[i][3..6]);
        cells[8].extend_from_slice(&sudoku_clone[i][6..9]);
    }

    let cell_row = line_nr / 3;
    let cell_col = col_nr / 3;

    for row in 0..3 {
            for col in 0..3 {

                if let Entry::Num(num) = sudoku_clone[row + cell_row * 3][col + cell_col * 3] {

                    inner_array[num as usize - 1] = false;

                    }

                }
                    
    }

    }
}
 
fn update_sudoku(sudoku: &mut [Line; 9]) {
    for line in sudoku {
        
        line
            .clone()
            .iter()
            .enumerate()
            .for_each( |(line_pos, entry)| {

            if let Entry::Empty(array) = entry {

                if array.iter().filter(|x| **x).count() == 1 {

                    let num = array.iter().position(|x| *x).unwrap();
                    line[line_pos] = Entry::Num(num as u32 + 1);
                    
                }
            }
            
        })
    }

}

fn unparse_sudoku(sudoku: Array2D<Entry>) -> String {
    let mut output = String::from("");

    for line in sudoku {
        for entry in line {
            if let Entry::Num(num) = entry { //entry must be the variant Entry::Num(_)
                output.push_str(&num.to_string())
            }
        }
        output.push('\n');
    }

    output.pop(); //Remove the last '\n'
    
    output
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_solution() {
        let contents = "\
__75__6_3
43___6__5
6_81_9_27
2_645____
__1_6_34_
7____8_5_
8__7__13_
_74_2_59_
1_93_5___";

        let expected_result = "\
917542683
432876915
658139427
296453871
581267349
743918256
865794132
374621598
129385764";

        let result = run(contents).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_parsing() {
        use Entry::*;
        let contents = "\
__75__6_3
__75__6_3
__75__6_3
__75__6_3
__75__6_3
__75__6_3
__75__6_3
__75__6_3
__75__6_3";

        let expected_result = [
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)]; 9
        ];

        let result = parse_contents(contents).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_analyze() {
        use Entry::*;
        let mut sudoku = [[Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)]; 9];
        sudoku[2][0] = Num(9); 
        sudoku[8][8] = Num(9);
        sudoku[8][6] = Num(8);
        sudoku[7][7] = Num(2);
        sudoku[7][6] = Num(5);
        sudoku[6][6] = Num(7);
        sudoku[6][7] = Num(1);
        sudoku[6][8] = Num(6);
//__75__6_3
//__75__6_3
//9_75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__716
//__75__523
//__75__8_9
        analyze(0, 0, &mut sudoku);
        analyze(8, 7, &mut sudoku);

        assert_eq!(sudoku[0][0], Empty([true, true, false, true, false, false, false, true, false]));
        assert_eq!(sudoku[8][7], Empty([false, false, false, true, false, false, false, false, false]));
    }
    
    #[test]
    fn quick_test() {

        let hej = 5;

        println!("{}", match hej {_ => true});

        let hej = vec![true; 2];

        println!("{:?}", hej);

        for i in 0..9 {
            println!("{}", i);
        }
    }
}