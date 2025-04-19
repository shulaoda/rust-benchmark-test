use std::collections::HashMap;

pub fn generate_cases(arr: &[&str]) -> [String; 4] {
    let mut dataset = HashMap::<usize, Vec<&str>>::new();

    for i in arr {
        if let Some(v) = dataset.get_mut(&i.len()) {
            v.push(i);
        } else {
            dataset.insert(i.len(), vec![i]);
        }
    }

    let mut king = 0;
    let mut end_result = String::new();

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

        if count > king {
            king = count;
            end_result = result.join("");
        }
    }

    end_result.replace_range(end_result.len() - 1.., "!");
    println!(
        "Generate worst case: {:?} -> len + O({}) -> O({})\n",
        end_result,
        king,
        king + arr.len()
    );

    [
        end_result,
        arr[0].to_string(),
        arr[arr.len() / 2].to_string(),
        arr[arr.len() - 1].to_string(),
    ]
}
