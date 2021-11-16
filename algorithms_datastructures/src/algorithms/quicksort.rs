use std::io;

// mod quicksort {
    fn partition(nums: &mut Vec<i32>, start: usize, end: usize) -> usize {
        let pivot = nums[start];
        let mut l = start + 1;
        let mut h = end;

        loop {
            while l <= h && nums[h] >= pivot {
                h -= 1;
            }
            while l <= h && nums[l] <= pivot {
                l += 1;
            }

            if l <= h {
                nums.swap(l, h);
            } else {
                break;
            }
        }

        nums.swap(start, h);
        return h;
    }

    fn quicksort(nums: &mut Vec<i32>, l: usize, h: usize) -> &mut Vec<i32> {
        if l >= h {
            return nums;
        } else {
            let partition_idx = partition(nums, l, h);
            quicksort(nums, l, partition_idx - 1);
            quicksort(nums, partition_idx + 1, h);
        }

        return nums;
    }

    pub fn quicksort_wrapper() {
        println!("Please input your list of numbers.");

        let mut numbers = String::new();

        // io::stdin()
        //     .read_line(&mut numbers)
        //     .expect("Failed to read line");

        // convert to array
        // let mut numbers: Vec<i32> = numbers
        //     .split_whitespace()
        //     .map(|s| s.parse().expect("parse error"))
        //     .collect();
        let mut numbers = vec![
            29, 99, 27, 41, 66, 28, 44, 78, 87, 19, 31, 76, 58, 88, 83, 97, 12, 21, 44,
        ];
        // let mut numbers = vec![7,5,4,3,2,1];

        // sort
        let n = numbers.len() - 1;
        let sorted_numbers = quicksort(&mut numbers, 0, n);

        // print result
        println!("Sorted: {:?}", sorted_numbers);
    }
// }

#[cfg(test)]
mod tests {
    use crate::algorithms::quicksort;

    #[test]
    fn test_quicksort() {
        quicksort::quicksort_wrapper();
    }

}