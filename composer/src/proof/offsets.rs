use bigint::U512;

/// Returns a vector of offsets that is read by an in-place lookup algorithm to determine the
/// location of a particular 32 byte value in the multiproof.
///
/// For more info, see: https://github.com/protolambda/eth-merkle-trees
pub fn calculate(indexes: Vec<U512>) -> Vec<u64> {
    let mut raw_indexes = vec![];

    // Convert indexes into arrays of bits
    for index in indexes.clone() {
        let mut bits = vec![0u8; 261];
        for i in 0..261 {
            bits[261 - i - 1] = index.bit(i) as u8;
        }

        raw_indexes.push(bits);
    }

    // Translate everything to an end node (padding with 1s from the right)
    let raw_indexes = raw_indexes
        .iter()
        .map(|index| {
            let mut index = index.clone();

            while index[0] == 0 {
                index.remove(0);
                index.push(1);
            }

            index
        })
        .collect();

    let mut ret: Vec<u64> = vec![indexes.len() as u64];
    ret.extend(helper(raw_indexes));
    ret
}

fn helper(indexes: Vec<Vec<u8>>) -> Vec<u64> {
    if indexes.len() <= 1 || indexes[0].len() == 0 {
        return vec![];
    }

    let mut left_subtree: Vec<Vec<u8>> = vec![];
    let mut right_subtree: Vec<Vec<u8>> = vec![];

    for mut index in indexes {
        let bit = index.remove(0);

        if bit == 0 {
            left_subtree.push(index);
        } else {
            right_subtree.push(index);
        }
    }

    let left_subtree_size = left_subtree.len() as u64;
    let left_subtree_offsets = helper(left_subtree);
    let right_subtree_offsets = helper(right_subtree);

    let mut ret = if left_subtree_size == 0 {
        vec![]
    } else {
        vec![left_subtree_size]
    };

    ret.extend(left_subtree_offsets);
    ret.extend(right_subtree_offsets);

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn offset_4_bit_left() {
        let indexes: Vec<U512> = vec![8.into(), 9.into(), 5.into(), 12.into(), 13.into(), 7.into()];
        assert_eq!(calculate(indexes), vec![6, 3, 2, 1, 2, 1]);
    }

    #[test]
    fn offset_4_bit_right() {
        let indexes: Vec<U512> = vec![
            4.into(),
            10.into(),
            11.into(),
            12.into(),
            13.into(),
            7.into(),
        ];

        assert_eq!(calculate(indexes), vec![6, 3, 1, 1, 2, 1]);
    }

    #[test]
    fn offset_4_bit_full() {
        let indexes: Vec<U512> = vec![
            8.into(),
            9.into(),
            10.into(),
            11.into(),
            12.into(),
            13.into(),
            14.into(),
            15.into(),
        ];

        assert_eq!(calculate(indexes), vec![8, 4, 2, 1, 1, 2, 1, 1]);
    }

    #[test]
    fn offset_4_bit_left_small_branch() {
        let indexes: Vec<U512> = vec![4.into(), 10.into(), 11.into(), 3.into()];
        assert_eq!(calculate(indexes), vec![4, 3, 1, 1]);
    }

    #[test]
    fn offset_4_bit_right_small_branch() {
        let indexes: Vec<U512> = vec![2.into(), 12.into(), 13.into(), 7.into()];
        assert_eq!(calculate(indexes), vec![4, 1, 2, 1]);
    }

    #[test]
    fn offset_5_bit_right_small_branch() {
        let indexes: Vec<U512> = vec![16.into(), 17.into(), 9.into(), 5.into(), 3.into()];
        assert_eq!(calculate(indexes), vec![5, 4, 3, 2, 1]);
    }

    #[test]
    fn offset_5_bit_left_branch() {
        let indexes: Vec<U512> = vec![
            16.into(),
            17.into(),
            9.into(),
            10.into(),
            11.into(),
            3.into(),
        ];

        assert_eq!(calculate(indexes), vec![6, 5, 3, 2, 1, 1]);
    }

    #[test]
    fn offset_5_bit_right_branch() {
        let indexes: Vec<U512> = vec![4.into(), 10.into(), 22.into(), 23.into(), 3.into()];
        assert_eq!(calculate(indexes), vec![5, 4, 1, 1, 1]);
    }

    #[test]
    fn offset_5_bit_full() {
        let mut indexes: Vec<U512> = vec![];

        for i in 16..32 {
            indexes.push(i.into());
        }

        assert_eq!(
            calculate(indexes),
            vec![16, 8, 4, 2, 1, 1, 2, 1, 1, 4, 2, 1, 1, 2, 1, 1]
        );
    }
}
