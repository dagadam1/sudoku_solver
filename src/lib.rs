// use ndarray::Array2;
// use ndarray::ShapeBuilder;
use ndarray::prelude::*;

#[macro_use]
extern crate ndarray;

// macro_rules! repeated_slice {
//     ([$($i:ident),*]; $n:literal) => {
//         $([$i])
//     };
// }


#[derive(Debug, PartialEq, Clone, Copy)]
enum Entry {
    Num(u32),
    Empty([bool; 9]), //Which values are possible
}

//type Line = [Entry; 9]; // [Line] = [[Entry]] line is a horizontal line and every entry is a number in the sudoku

pub fn run(contents: &str) -> Result<String, String> {
    let mut parsed = parse_contents(contents)?;
    
    parsed = solve(parsed)?;

    Ok(unparse_sudoku(parsed))
}

fn parse_contents(contents: &str) -> Result<Array2<Entry>, String> {
    const RADIX: u32 = 10;

    let mut vec: Vec<Entry> = vec![];

    for line in contents.lines() {
        if line.len() != 9 {
            return Err(format!("Expected a 9x9 sudoku but got a line length of '{}' instead!", line.len()));
        }
        let mut vec = vec![];
        //Fill vec with the characters from the input
        line.chars()
            .for_each(|character| {
            vec.push(match character.to_digit(RADIX) {
                Some(num) => Entry::Num(num),
                None => Entry::Empty([true; 9]),
            })
        });
    }

    //Fill Vec<Line>
    // for line in contents.lines() {
    //     if line.len() != 9 {
    //         return Err(format!("Expected a 9x9 sudoku but got a line length of '{}' instead!", line.len()));
    //     }
    //     let mut line_vec = vec![];
    //     for character in line.chars() {
    //         line_vec.push(
    //             match character.to_digit(RADIX) {
    //             Some(num) => Entry::Num(num),
    //             None => Entry::Empty([true; 9]),
    //         });
    //     };
    //     vec.push(line_vec);
    // };

    let array = Array2::from_shape_vec((9, 9), vec).map_err(|err| err.to_string())?;
    Ok(array)
}

fn solve(sudoku: Array2<Entry>) -> Result<Array2<Entry>, String> {
    let mut iterations = 0;
    // Loop while sudoku contains empty entries i.e. is unsolved
    while sudoku.iter()
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

    Ok(sudoku)
}

fn analyze(line_nr: usize, col_nr: usize, sudoku: Array2<Entry>) -> Array2<Entry>{
    let sudoku_clone = sudoku.clone();

    if let Entry::Empty(ref mut inner_array) = sudoku[(line_nr, col_nr)] {

    //Check line
    let line = sudoku.as_rows()[line_nr];
    for entry in line {
        if let Entry::Num(num) = entry {
            inner_array[num as usize - 1] = false;
        }
    }

    //Check column
    let column = sudoku.as_columns()[col_nr];
    for entry in column {
        if let Entry::Num(num) = entry {
            inner_array[num as usize - 1] = false;
        }
    }

    //One cell is one of the 9 3x3 grids in the sudoku
    //Left to right, up to down
    let mut cells: Vec<Vec<Entry>> = vec![Vec::with_capacity(9); 9];

    for i in 0..3 {
        let row = sudoku.row_iter(i);
        cells[0].append(&mut row.take(3).cloned().collect());
        cells[1].append(&mut row.take(3).cloned().collect());
        cells[2].append(&mut row.take(3).cloned().collect());
    }
    for i in 3..6 {
        let row = sudoku.row_iter(i);
        cells[3].append(&mut row.take(3).cloned().collect());
        cells[4].append(&mut row.take(3).cloned().collect());
        cells[5].append(&mut row.take(3).cloned().collect());
    }
    for i in 6..9 {
        let row = sudoku.row_iter(i);
        cells[6].append(&mut row.take(3).cloned().collect());
        cells[7].append(&mut row.take(3).cloned().collect());
        cells[8].append(&mut row.take(3).cloned().collect());
    }

    let cell_row = line_nr / 3;
    let cell_col = col_nr / 3;

    for row in 0..3 {
            for col in 0..3 {

                if let Entry::Num(num) = sudoku[(row + cell_row * 3, col + cell_col * 3)] {

                    inner_array[num as usize - 1] = false;

                    }

                }
                    
    }

    }

    sudoku
}
 
fn update_sudoku(sudoku: Array2<Entry>) {
    for line in sudoku.as_rows() {
        
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

fn unparse_sudoku(sudoku: Array2<Entry>) -> String {
    let mut output = String::from("");

    for line in sudoku.rows_iter() {
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

    let expected_result_vec = vec![];

    for i in 0..9 {
        expected_result_vec.append(
            &mut vec![Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)]
        )
    }

    let expected_result: Array2<Entry> = Array2::from_row_major(&expected_result_vec, 9, 9);

    println!("{:?}", expected_result);

    let result = parse_contents(contents).unwrap();

    assert_eq!(result, expected_result);

    }

    #[test]
    fn test_analyze() {
        use Entry::*;

        let sudoku_vec = vec![];

        for i in 0..9 {
            sudoku_vec.append(
                &mut vec![Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)]
            )
        }
        let mut sudoku = Array2::from_row_major(&sudoku_vec, 9, 9);
        sudoku[(2, 0)] = Num(9); 
        sudoku[(8, 8)] = Num(9);
        sudoku[(8, 6)] = Num(8);
        sudoku[(7, 7)] = Num(2);
        sudoku[(7, 6)] = Num(5);
        sudoku[(6, 6)] = Num(7);
        sudoku[(6, 7)] = Num(1);
        sudoku[(6, 8)] = Num(6);
//__75__6_3
//__75__6_3
//9_75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__716
//__75__523
//__75__8_9
        analyze(0, 0, sudoku);
        analyze(8, 7, sudoku);

        assert_eq!(sudoku[(0, 0)], Empty([true, true, false, true, false, false, false, true, false]));
        assert_eq!(sudoku[(8, 7)], Empty([false, false, false, true, false, false, false, false, false]));
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