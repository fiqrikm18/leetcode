use std::collections::HashMap;

pub struct TwoSumSolution;

impl TwoSumSolution {
    pub fn two_sum(&self, nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut result: Vec<i32> = Vec::new();
        let map = self.convert_vec_to_hash_map(nums.clone());

        for (idx, num) in nums.into_iter().enumerate() {
            let compliment = target - num;
            let res = map.get(&compliment).copied().unwrap_or(0);
            if res != 0 && res != idx as i32 {
                result.push(idx as i32);
                result.push(res);

                break;
            }
        }

        result
    }

    fn convert_vec_to_hash_map(&self, nums: Vec<i32>) -> HashMap<i32, i32> {
        let mut map: HashMap<i32, i32> = HashMap::new();
        for (idx, val) in nums.into_iter().enumerate() {
            let index: i32 = idx.try_into().unwrap();
            map.insert(val, index);
        }

        map
    }
}

pub fn solve() {
    let two_sum_solution = TwoSumSolution {};
    let target = 9;
    let nums = vec![2, 7, 11, 15];
    let result = two_sum_solution.two_sum(nums, target);

    println!("{result:?}");
}
