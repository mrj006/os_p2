use crate::models::count;

pub fn count_join(values: count::CountJoinInput) -> usize {
    values.values.iter().sum()
}
