use std::borrow::BorrowMut;

#[derive(Debug, PartialEq, Clone)]
enum Cell {
    Num(u32),
    Empty(Vec<bool>), //Which values are possible
}

pub fn run(contents: &str) -> String {
    let parsed = parse_contents(contents);
    
    solve(parsed)
}

fn parse_contents(contents: &str) -> Vec<Vec<Cell>> {
    const RADIX: u32 = 10;

    // //Find largest number. JUST ASSUME 9
    //println!("{}", contents.chars().map(|x| x.to_digit(RADIX)).filter_map(|x| x).max().unwrap());

    //Construct Vec<Vec<Cell>>
    let mut vec: Vec<Vec<Cell>> = vec![];

    for line in contents.lines() {

        vec.push(line.chars().map(|x| {

            match x.to_digit(RADIX) {
                Some(num) => Cell::Num(num),
                None => Cell::Empty(vec![true; 9]),
            }

        }).collect())

    };

    vec
    

    // //This whole section is pretty unreadable but could be remade later (maybe with loops) if it causes problems
    // //Find largest number
    // let primary: Map<Vec<Option<u32>>> = lines.map(|line| 
    //     { line.chars().map(|character| character.to_digit(RADIX)).collect() });



    // let outer_vec: Vec<Vec<Cell>> = primary.map(|number: Option<u32>|
    // { 
    //     match number {
    //         Some(num) => Cell::Num(num),
    //         None => Cell::Empty(vec![true; ]),
    //     } 
    // }).collect();

    // outer_vec
}

fn solve(sudoku: Vec<Vec<Cell>>) -> String {
    for line in sudoku {
        let line_clone = line.to_vec();

        for cell in line.into_iter() {
            if let Cell::Empty(mut vec) = cell {

                line_clone
                .iter()
                .filter(|x| match x
                    { 
                        Cell::Empty(_) => false,
                        Cell::Num(_) => true,
                    })
                .for_each(|x| 
                    { 
                        if let Cell::Num(num) = x {
                            vec[*num as usize] = false;
                        }
                    });
            }
        }
    }

    todo!();
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

        let result = run(contents);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_parsing() {
        use Cell::*;
        let contents = "\
__75__6_3
43___6__5";

        let expected_result = vec![
            vec![Empty(vec![true; 9]),Empty(vec![true; 9]),Num(7),Num(5),Empty(vec![true; 9]),Empty(vec![true; 9]),Num(6),Empty(vec![true; 9]),Num(3)],
            vec![Num(4),Num(3),Empty(vec![true; 9]),Empty(vec![true; 9]),Empty(vec![true; 9]),Num(6),Empty(vec![true; 9]),Empty(vec![true; 9]),Num(5)],
        ];

        let result = parse_contents(contents);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn quick_test() {

        let hej = 5;

        println!("{}", match hej {_ => true});

        let hej = vec![true; 2];

        println!("{:?}", hej);
    }
}