use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Entry {
    Num(u32),
    Empty([bool; 9]), //Which values are possible
}

type Line = [Entry; 9]; // [Line] = [[Entry]] line is a horizontal line and every entry is a number in the sudoku

pub fn run(contents: &str) -> Result<String, String> {
    let mut parsed = parse_contents(contents)?;
    
    Ok(solve(&mut parsed))
}

fn parse_contents(contents: &str) -> Result<[Line; 9], String> {
    const RADIX: u32 = 10;

    let mut vec: Vec<Line> = vec![];

    //Fill Vec<Line>
    for line in contents.lines() {
        if line.len() != 9 {
            return Err(format!("Expected a 9x9 sudoku but got a line length of '{}' instead!", line.len()));
        }
        let mut line_array: Line = [Entry::Num(0); 9];
        for (i, character) in line.chars().enumerate() {
            line_array[i] = match character.to_digit(RADIX) {
                Some(num) => Entry::Num(num),
                None => Entry::Empty([true; 9]),
            };
        };
        vec.push(line_array);
    };
    
    let array: [Line; 9] = vec.try_into().map_err(|_| "Expected 9 lines!")?;
    Ok(array)
}

fn solve(sudoku: &mut [Line; 9]) -> String {
    for line_nr in 0..9 {
        for col_nr in 0..9 {
            analyze(line_nr, col_nr, sudoku);
        }
    }

    update_sudoku(sudoku);
    
    println!("{:?}", sudoku);
    todo!();
}

fn analyze(line_nr: usize, col_nr: usize, sudoku: &mut [Line; 9]) {
    let sudoku_clone = sudoku.clone();
    
    if let Entry::Empty(ref mut inner_array) = sudoku[line_nr][col_nr] {

        //Check line
        for i in 0..9 {
            if let Entry::Num(num) = sudoku_clone[line_nr][i] {
                inner_array[num as usize - 1] = false;
            }
        }

        //Check column
        for i in 0..9 {
            if let Entry::Num(num) = sudoku_clone[i][col_nr] {
                inner_array[num as usize - 1] = false;
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
//__75__6_3
//__75__6_3
//9_75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3

        analyze(0, 0, &mut sudoku);

        println!("{:?}", sudoku);

        assert_eq!(sudoku[0][0], Empty([true, true, false, true, false, false, false, true, false]));
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