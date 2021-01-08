fn solution(target: i32, nums: &[i32]) -> [i32; 2] {
    let mut res = [0, 0];
    
    let mut k = 0;
    for i in nums {
        if k >= nums.len() {
            break;
        }

        k += 1;

        let mut l = 0;
        for j in &nums[k..] {
            if target == i + j {
                res[0] = (k - 1) as i32;
                res[1] = (l + k) as i32;
            }
            l += 1;
        }
    }

    res
}

struct Solution;

impl Solution {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut res: Vec<i32> = Vec::new();
        for (k1, v1) in nums.iter().enumerate() {
            if k1 >= nums.len() {
                break;
            }
            for (k2, v2) in nums[k1+1..].iter().enumerate() {
                if (target == v1 + v2) {
                    res.push(k1 as i32);
                    res.push((k1 + 1 + k2) as i32);
                }
            }
        }

        res
    }
}

#[test]
fn two_sum() {
    let nums1 = [2, 7, 11, 15];
    let target1 = 9;
    assert_eq!([0, 1], solution(target1, &nums1));

    let nums2 = [3, 2, 4];
    let target2 = 6;
    assert_eq!([1, 2], solution(target2, &nums2));

    let nums3 = [3, 3];
    let target3 = 6;
    assert_eq!([0, 1], solution(target3, &nums3));
}

#[test]
fn two_sum_ob() {
    let nums1 = vec![2, 7, 11, 15];
    let target1 = 9;

    assert_eq!(vec![0, 1], Solution::two_sum(nums1, target1));

    let nums2 = vec![3, 2, 4];
    let target2 = 6;
    assert_eq!(vec![1, 2], Solution::two_sum(nums2, target2));

    let nums3 = vec![3, 3];
    let target3 = 6;
    assert_eq!(vec![0, 1], Solution::two_sum(nums3, target3));
}
