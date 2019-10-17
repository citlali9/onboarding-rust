use onboarding_rust::week2::exercise38::subdomain_visits;

#[test]
fn one_subdomine_visits() {
    let input = vec![
        "9001 discuss.leetcode.com".to_string(),
        "9 discuss.leetcode.com".to_string(),
    ];
    let output = vec![
        "9010 discuss.leetcode.com".to_string(),
        "9010 leetcode.com".to_string(),
        "9010 com".to_string(),
    ];
    assert!(contains_all(output, subdomain_visits(input)));
}

#[test]
fn two_subdomine_visits() {
    let input = vec![
        "900 google.mail.com".to_string(),
        "50 yahoo.com".to_string(),
        "1 intel.mail.com".to_string(),
        "5 wiki.org".to_string(),
    ];
    let output = vec![
        "901 mail.com".to_string(),
        "50 yahoo.com".to_string(),
        "900 google.mail.com".to_string(),
        "5 wiki.org".to_string(),
        "5 org".to_string(),
        "1 intel.mail.com".to_string(),
        "951 com".to_string(),
    ];
    assert!(contains_all(output, subdomain_visits(input)));
}

fn contains_all(a: Vec<String>, b: Vec<String>) -> bool {
    a.len() == b.len() && a.iter().all(|x| b.contains(x))
}