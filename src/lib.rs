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

fn solve(mut sudoku: Array2<Entry>) -> Result<Array2<Entry>, String> {
    let mut iterations = 0;
    // Loop while sudoku contains empty entries i.e. is unsolved
    while sudoku.iter()
                .any(|entry| if let Entry::Empty(_) = entry { true } else { false } ) {
        
        for line_nr in 0..9 {
            for col_nr in 0..9 {
                sudoku = analyze(line_nr, col_nr, sudoku);
            }
        }

        sudoku = update_sudoku(sudoku);

        if iterations >= 20 {
            return Err(String::from("Could not find solution!"));
        }

        iterations += 1;
    }

    Ok(sudoku)
}

fn analyze(line_nr: usize, col_nr: usize, mut sudoku: Array2<Entry>) -> Array2<Entry>{
    let sudoku_clone = sudoku.clone();

    if let Entry::Empty(ref mut inner_array) = sudoku[(line_nr, col_nr)] {

        //Check line
        //There can only be one of each number in each line
        let line = sudoku_clone.row(line_nr);
        for entry in line {
            if let Entry::Num(num) = entry {
                inner_array[*num as usize - 1] = false;
            }
        }

        //Check column
        //There can only be one of each number in each column
        let column = sudoku_clone.column(col_nr);
        for entry in column {
            if let Entry::Num(num) = entry {
                inner_array[*num as usize - 1] = false;
            }
        }



        //One cell is one of the 9 3x3 parts of the sudoku
        let cells = sudoku_clone.exact_chunks((3, 3));

        let cell_row = line_nr / 3;
        let cell_col = col_nr / 3;
        
        //The cell that this number is in
        let current_cell = cells.into_iter().nth(cell_row * 3 + cell_col).unwrap(); //Not sure about this

        //For every number in current_cell, make the inner array of the Entry::Empty false
        //Because there can only be one of each number in every cell of the sudoku
        current_cell.iter().for_each(|entry| {
            if let Entry::Num(num) = entry {
                inner_array[*num as usize - 1] = false;
            }
        });

    }

    sudoku
}
 
fn update_sudoku(mut sudoku: Array2<Entry>) -> Array2<Entry> {
    for mut line in sudoku.rows_mut() {
        line.map_inplace(|entry| {
            if let Entry::Empty(array) = entry {
                //Check if there is only one possible number that this entry can be
                if array.iter().filter(|x| **x).count() == 1 {
                    //Check which index has the value of true and thus is the possible number
                    let num = array.iter().position(|x| *x).unwrap();
                    *entry = Entry::Num(num as u32);
                }
            }
        });
    }
    sudoku

}

fn unparse_sudoku(sudoku: Array2<Entry>) -> String {
    let mut output = String::from("");

    for line in sudoku.rows() {
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

        let expected_result1 = array![
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],
            [Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)],];

        let mut vec = vec![];

        for _ in 0..9 {
            vec.append(
                &mut vec![Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)]
            )
        }

        let expected_result = Array2::from_shape_vec((9, 9), vec).unwrap();

        println!("{:?}", expected_result);

        assert_eq!(expected_result1, expected_result);

    

        let result = parse_contents(contents).unwrap();

        assert_eq!(result, expected_result);

    }

    #[test]
    fn test_analyze() {
        use Entry::*;

        let mut sudoku_vec = vec![];

        for _ in 0..9 {
            sudoku_vec.append(
                &mut vec![Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)]
            )
        }
        let mut sudoku: Array2<Entry> = Array2::from_shape_vec((9, 9), sudoku_vec).unwrap();
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
        sudoku = analyze(0, 0, sudoku);
        sudoku = analyze(8, 7, sudoku);

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