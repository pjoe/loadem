use std::f32::NAN;

pub fn sort(data: &mut Vec<f32>) {
    data.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
}

pub fn extend_sorted(data: &mut Vec<f32>, new_data: &[f32]) {
    // reverse "merge sort" O(N), N = len(new_data)
    let mut old_idx = data.len();
    data.resize(data.len() + new_data.len(), 0.0);
    let mut new_idx = new_data.len();
    let mut idx = data.len();

    while old_idx > 0 && new_idx > 0 {
        let old = data[old_idx - 1];
        let new = new_data[new_idx - 1];
        if old > new {
            data[idx - 1] = old;
            old_idx -= 1;
        } else {
            data[idx - 1] = new;
            new_idx -= 1;
        }
        idx -= 1;
    }

    while old_idx > 0 {
        data[idx - 1] = data[old_idx - 1];
        old_idx -= 1;
        idx -= 1;
    }
    while new_idx > 0 {
        data[idx - 1] = new_data[new_idx - 1];
        new_idx -= 1;
        idx -= 1;
    }
}

pub fn percentile(data: &[f32], pct: f32) -> f32 {
    assert!(pct >= 0.0);
    let hundred: f32 = 100.0;
    assert!(pct <= hundred);
    match data.len() {
        0 => NAN,
        1 => data[0],
        len => {
            if (pct - hundred).abs() < f32::EPSILON {
                return data[len -1];
            }
            let length = (len - 1) as f32;
            let rank = (pct / hundred) * length;
            let lrank = rank.floor();
            let d = rank - lrank;
            let n = lrank as usize;
            let lo = data[n];
            let hi = data[n + 1];
            lo + (hi - lo) * d
        }
    }
}


#[cfg(test)]
mod tests {
    use super::{sort, extend_sorted, percentile};

    #[test]
    fn it_works() {
        let mut stats = Vec::<f32>::new();

        let mut new_data = vec![2.0, 3.2, 1.8];
        sort(&mut new_data);
        extend_sorted(&mut stats, &new_data);
        assert_eq!(stats, vec![1.8, 2.0, 3.2]);

        new_data = vec![3.8, 0.9, 2.4];
        sort(&mut new_data);
        extend_sorted(&mut stats, &new_data);
        assert_eq!(stats, vec![0.9, 1.8, 2.0, 2.4, 3.2, 3.8]);

        assert_eq!(percentile(&stats, 50.0), 2.2);
        assert_eq!(percentile(&stats, 0.0), 0.9);
        assert_eq!(percentile(&stats, 100.0), 3.8);
    }
}
