use std::collections::HashMap;

pub fn generate_cases(arr: &[&str]) -> [String; 5] {
    let mut dataset = HashMap::<usize, Vec<&str>>::new();

    for i in arr {
        dataset.entry(i.len()).or_default().push(i);
    }

    let mut king = 0;
    let mut final_result = String::new();

    let multi_times = dataset
        .iter()
        .max_by(|x, y| {
            let result = x.1.len().cmp(&y.1.len());
            if result.is_eq() { x.0.cmp(y.0) } else { result }
        })
        .map(|(_, i)| i.last().unwrap().to_string())
        .unwrap_or_default();

    for value in &mut dataset.values().clone() {
        let mut strings = value.clone();

        let mut count = 0;
        let mut result = vec![];

        loop {
            if strings.is_empty() {
                break;
            }

            let mut prefix_count: HashMap<&str, usize> = HashMap::new();

            for s in strings {
                for i in 1..=s.len() {
                    let prefix = &s[..i];
                    *prefix_count.entry(prefix).or_insert(0) += 1;
                }
            }

            let prefix = prefix_count
                .iter()
                .max_by_key(|(prefix, count)| prefix.len() * *count)
                .map(|(prefix, _)| prefix.to_string())
                .unwrap_or_default();

            let max = prefix_count.get(prefix.as_str()).unwrap() * prefix.len();
            strings = prefix_count
                .keys()
                .filter_map(|key| {
                    key.strip_prefix(&prefix)
                        .and_then(|v| (!v.is_empty()).then_some(v))
                })
                .collect::<Vec<&str>>();

            count += max;
            result.push(prefix);
        }

        if king < count {
            king = count;
            final_result = result.join("");
        }
    }

    final_result.replace_range(final_result.len() - 1.., "!");
    println!(
        "Generate multi-times case: {} \nGenerate worst case: {} -> len + O({}) -> O({})\n",
        multi_times,
        final_result,
        king,
        king + arr.len()
    );

    [
        multi_times,
        final_result,
        arr[0].to_string(),
        arr[arr.len() / 2].to_string(),
        arr[arr.len() - 1].to_string(),
    ]
}

#[test]
fn test() {
    let case1 = ["ab", "abc"];
    let case2 = ["ab", "abc", "ad", "af"];
    let case3 = ["a", "ab", "abc", "abcd", "abcde"];

    generate_cases(&case1);
    generate_cases(&case2);
    generate_cases(&case3);
}
