use std::ops::SubAssign;
pub fn check_and_use<T, K>(resource: &mut T, cost: K) -> bool
    where T: SubAssign<K> + PartialOrd<K>
{
    if *resource >= cost {
        *resource -= cost;
        true
    } else {
        false
    }
}
