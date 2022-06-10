use std::convert::TryInto;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Entry {
    Num(u32),
    Empty([bool; 9]), //Which values are possible
}

type Line = [Entry; 9]; // [Line] = [[Entry]] line is a horizontal line and every entry is a number in the sudoku

pub fn run(contents: &str) -> Result<String, String> {
    let parsed = parse_contents(contents)?;
    
    Ok(solve(parsed))
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
    

    // //This whole section is pretty unreadable but could be remade later (maybe with loops) if it causes problems
    // //Find largest number
    // let primary: Map<Vec<Option<u32>>> = lines.map(|line| 
    //     { line.chars().map(|character| character.to_digit(RADIX)).collect() });



    // let outer_vec: Vec<Vec<Entry>> = primary.map(|number: Option<u32>|
    // { 
    //     match number {
    //         Some(num) => Entry::Num(num),
    //         None => Entry::Empty(vec![true; ]),
    //     } 
    // }).collect();

    // outer_vec
}

fn solve(sudoku: [Line; 9]) -> String {
    for line in sudoku {
        let line_clone = line.to_vec();

        for (entry_nr, entry) in line.iter().enumerate() {
            if let Entry::Empty(mut vec) = entry {

                line_clone
                .iter()
                .filter(|x| match x
                    { 
                        Entry::Empty(_) => false,
                        Entry::Num(_) => true,
                    })
                .for_each(|x| 
                    { 
                        if let Entry::Num(num) = x {
                            vec[*num as usize - 1] = false;
                        }
                    });

                for i in 0..9 {
                    if let Entry::Num(num) = sudoku[i][entry_nr] {
                        vec[num as usize - 1] = false;
                    }
                }
            }
        }
    }

    todo!();
}

fn analyze(position: (usize, usize), sudoku: [Line; 9]) -> [Line; 9] {
    let (ln, col) = position;
    
    if let Entry::Empty(mut inner_array) = sudoku[ln][col] {
        sudoku[ln]
            .iter()
            .for_each(|x|
        {
            if let Entry::Num(num) = x {
                println!("{}", &num);
                inner_array[*num as usize - 1] = false;
            } 
        });
        
        for i in 0..9 {
            if let Entry::Num(num) = sudoku[i][col] {
                inner_array[num as usize - 1] = false;
            }
        }
    }
    sudoku
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
    fn test_analysis() {
        use Entry::*;
        let sudoku = [[Empty([true; 9]),Empty([true; 9]),Num(7),Num(5),Empty([true; 9]),Empty([true; 9]),Num(6),Empty([true; 9]),Num(3)]; 9];
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3
//__75__6_3

        let result = analyze((0,0), sudoku);

        println!("{:?}", result);
        println!("{:?}", sudoku);
        println!("{:?}", result == sudoku);

        assert_eq!(result[0][0], Empty([true, true, false, true, false, false, false, true, true]));
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