use onboarding_rust::week2::exercise47::valid_sudoku;

#[test]
fn sodoku_one_test() {
	let input = vec![
		vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
		vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
		vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
		vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
		vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
		vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
		vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
		vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
		vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
	];
	let expected = true;
	assert_eq!(expected, valid_sudoku(input));
}

#[test]
fn sodoku_two_test() {
	//wrong col
	let input = vec![
		vec!['8', '3', '.', '.', '7', '.', '.', '.', '.'],
		vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
		vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
		vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
		vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
		vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
		vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
		vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
		vec!['.', '.', '.', '.', '8', '.', '.', '7', '9'],
	];
	let expected = false;
	assert_eq!(expected, valid_sudoku(input));
}

#[test]
fn sodoku_three_test() {
	//wrong box
	let input = vec![
		vec!['.', '.', '.', '.', '.', '.', '5', '.', '.'],
		vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
		vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
		vec!['9', '3', '.', '.', '2', '.', '4', '.', '.'],
		vec!['.', '.', '7', '.', '.', '.', '3', '.', '.'],
		vec!['.', '.', '.', '.', '.', '.', '.', '.', '.'],
		vec!['.', '.', '.', '3', '4', '.', '.', '.', '.'],
		vec!['.', '.', '.', '.', '.', '3', '.', '.', '.'],
		vec!['.', '.', '.', '.', '.', '5', '2', '.', '.'],
	];
	let expected = false;
	assert_eq!(expected, valid_sudoku(input));
}
