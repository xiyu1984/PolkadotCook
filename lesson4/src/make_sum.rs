pub fn checked_sum(numbers: &[u32]) -> Option<u32> {
    let mut sum: u32= 0;
    for ele in numbers {
        if let Some(val) = sum.checked_add(*ele) {
            sum = val;
        } else {
            return None;
        }
    }

    Some(sum)
}