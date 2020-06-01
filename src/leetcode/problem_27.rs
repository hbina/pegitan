use crate::Solution;

impl Solution {
    //  TODO    ::  You should be able to find the solution in O(n) complexity time.
    //              The idea is to just iterate through nums, find an matching element
    //              then try to find a replacement value from element
    pub fn remove_element(_nums: &mut Vec<i32>, _val: i32) -> i32 {
        let fix_pointer = 0;
        fix_pointer
    }

    pub fn remove_element_two_pointer(nums: &mut Vec<i32>, val: i32) -> i32 {
        // Overflow??
        let mut back_pointer = nums.len() - 1;
        println!("began back pointer at {}", back_pointer);
        while nums[back_pointer] == val {
            if back_pointer == 0 {
                break;
            } else {
                back_pointer -= 1;
            }
        }
        println!("brought back pointer to {}", back_pointer);
        let mut front_pointer = 0;
        while front_pointer < back_pointer {
            println!(
                "front pointer:{} back pointer: {}",
                front_pointer, back_pointer
            );
            // Do the checks...
            if nums[front_pointer] == val {
                // Clear up back pointer
                while nums[back_pointer] == val {
                    if back_pointer == 0 {
                        break;
                    } else {
                        back_pointer -= 1;
                    }
                }
                if front_pointer > back_pointer {
                    // Means we have cleaned up everything
                    break;
                } else {
                    let tmp = nums[front_pointer];
                    nums[front_pointer] = nums[back_pointer];
                    nums[back_pointer] = tmp;
                }
            }
            front_pointer += 1;
        }
        front_pointer as i32
    }
}

#[test]
pub fn problem_27_test() {
    assert_eq!(
        Solution::remove_element_two_pointer(vec![3, 2, 2, 3].as_mut(), 3),
        2
    );
}
