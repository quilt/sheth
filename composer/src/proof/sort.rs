use bigint::U512;
use std::ops::Shl;

/// Sort a vector bit-alphabetically
///
/// For more info, see: https://github.com/ethereum/eth2.0-specs/issues/1303
pub fn alpha_sort(n: &Vec<U512>) -> Vec<U512> {
    let mut ret = n.clone();

    ret.sort_by(|a, b| {
        let (a, a_shift, b, b_shift) = normalize(*a, *b);
        match a.cmp(&b) {
            std::cmp::Ordering::Less => std::cmp::Ordering::Less,
            std::cmp::Ordering::Greater => std::cmp::Ordering::Greater,
            std::cmp::Ordering::Equal => a_shift.cmp(&b_shift),
        }
    });

    ret
}

fn normalize(a: U512, b: U512) -> (U512, usize, U512, usize) {
    // Normalize (e.g. right pad until the the most significant bit in `a` and `b` align)
    let max = std::cmp::max(a.bits(), b.bits());

    let (a, a_shift) = if a.bits() < max {
        let shift = max - a.bits();
        (a.shl(shift), shift)
    } else {
        (a, 0)
    };

    let (b, b_shift) = if b.bits() < max {
        let shift = max - b.bits();
        (b.shl(shift), shift)
    } else {
        (b, 0)
    };

    (a, a_shift, b, b_shift)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn normalize_numbers() {
        let one = U512::from(1);
        let two = U512::from(2);
        let big = U512::from(std::u64::MAX);

        assert_eq!(normalize(one, two), (two, 1, two, 0));
        assert_eq!(normalize(big, one), (big, 0, U512::from(2u64.pow(63)), 63));
    }

    #[test]
    fn alpha_sort_two_numbers() {
        assert_eq!(
            alpha_sort(&vec![3.into(), 2.into()]),
            vec![2.into(), 3.into()]
        );
    }

    #[test]
    fn alphas_sort_branch() {
        let unsorted: Vec<U512> = vec![20, 21, 11, 4, 3]
            .into_iter()
            .fold(vec![], |mut acc, n| {
                acc.push(n.into());
                acc
            });

        let sorted: Vec<U512> = vec![4, 20, 21, 11, 3]
            .into_iter()
            .fold(vec![], |mut acc, n| {
                acc.push(n.into());
                acc
            });

        assert_eq!(alpha_sort(&unsorted), sorted);
    }

    #[ignore] // Current implementation only works on branches
    #[test]
    fn alpha_sort_many_numbers() {
        let unsorted: Vec<U512> = (1..4).fold(vec![], |mut acc, n| {
            acc.push(n.into());
            acc
        });

        let sorted: Vec<U512> = vec![
            16, 8, 17, 4, 18, 9, 19, 2, 20, 10, 21, 5, 22, 11, 23, 1, 24, 12, 25, 6, 26, 13, 27, 3,
            28, 14, 29, 7, 30, 15, 31,
        ]
        .into_iter()
        .fold(vec![], |mut acc, n| {
            acc.push(n.into());
            acc
        });

        assert_eq!(alpha_sort(&unsorted), sorted);
    }
}
