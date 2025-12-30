use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, num1) in nums.iter().enumerate() {
        for (j, num2) in nums.iter().enumerate() {
            if i == j {
                continue;
            }
            if num1 + num2 == target {
                let answer = vec![i as i32, j as i32];
                return answer;
            }
        }
    }
    vec![]
}

pub fn two_sum_answer(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;

        if let Some(&j) = map.get(&complement) {
            return vec![j as i32, i as i32];
        }

        map.insert(num, i);
    }

    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1_two_sum() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn test2_two_sum() {
        let nums = vec![3, 2, 4];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![1, 2]);
    }

    #[test]
    fn test3_two_sum() {
        let nums = vec![3, 3];
        let target = 6;
        let result = two_sum(nums, target);
        assert_eq!(result, vec![0, 1]);
    }
}
