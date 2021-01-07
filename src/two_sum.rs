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

#[test]
fn two_sum_test() {
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
