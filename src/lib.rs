use std::iter::Map;


#[derive(Debug, PartialEq)]
enum Cell {
    Num(u32),
    Empty(Vec<bool>), //Which values are possible
}

fn run(contents: &str) -> String {
    let parsed = parse_contents(contents);
    
    solve(parsed)
}

fn parse_contents(contents: &str) -> Vec<Vec<Cell>> {
    const RADIX: u32 = 10;

    let mut vec: Vec<Vec<Cell>> = vec![];

    for line in contents.lines() {

        vec.push(line.chars().map(|x| {

            match x.to_digit(RADIX) {
                Some(num) => Cell::Num(num),
                None => Cell::Empty(vec![]),
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
    todo!();
}


#[cfg(test)]
mod tests {
    use std::result;

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
            vec![Empty(vec![]),Empty(vec![]),Num(7),Num(5),Empty(vec![]),Empty(vec![]),Num(6),Empty(vec![]),Num(3)],
            vec![Num(4),Num(3),Empty(vec![]),Empty(vec![]),Empty(vec![]),Num(6),Empty(vec![]),Empty(vec![]),Num(5)],
        ];

        let result = parse_contents(contents);

        assert_eq!(result, expected_result);
    }
}