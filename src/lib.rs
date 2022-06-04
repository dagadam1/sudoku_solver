fn parse_contents(contents: &str) -> Vec<Vec<i8>> {
    todo!();
}

fn solve(sudoku: Vec<Vec<i8>>) -> String {
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

        let parsed = parse_contents(contents);
        let result = solve(parsed);

        assert_eq!(result, expected_result);
    }
}