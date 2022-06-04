fn run(contents: &str) -> String {
    let parsed = parse_contents(contents);
    
    solve(parsed)
}

fn parse_contents(contents: &str) -> Vec<Vec<Option<u32>>> {
    const RADIX: u32 = 10;

    let lines = contents.lines();
    let outer_vec: Vec<Vec<Option<u32>>> = lines.map(|line| { line.chars().map(|character| { character.to_digit(RADIX) }).collect() }).collect();

    outer_vec
}

fn solve(sudoku: Vec<Vec<Option<u32>>>) -> String {
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
        let contents = "\
__75__6_3
43___6__5";

        let expected_result = vec![
            vec![None,None,Some(7),Some(5),None,None,Some(6),None,Some(3)],
            vec![Some(4),Some(3),None,None,None,Some(6),None,None,Some(5)],
        ];

        let result = parse_contents(contents);

        assert_eq!(result, expected_result);
    }
}