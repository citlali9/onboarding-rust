use onboarding_rust::week2::exercise37::find_the_difference;

#[test]
fn one_find_the_letter() {
    let s = "abcd";
    let t = "abcde";
    let ch = 'e';
    assert_eq!(ch, find_the_difference(s, t));
}

#[test]
fn two_find_the_letter() {
    let s = "ababa";
    let t = "abbaba";
    let ch = 'b';
    assert_eq!(ch, find_the_difference(s, t));
}