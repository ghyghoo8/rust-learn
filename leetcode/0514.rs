/*
* 练手题：https://leetcode-cn.com/problems/string-compression/  
* https://leetcode-cn.com/problems/bu-ke-pai-zhong-de-shun-zi-lcof/
* 附加题：https://leetcode-cn.com/problems/minimum-number-of-frogs-croaking/
*/
impl Solution {
    pub fn is_straight(nums: Vec<i32>) -> bool {
        // nums.sort();
        let mut nums2 = nums.clone(); //克隆一个vec
        nums2.sort();//排个序
        println!("{:?}", nums2);
        let mut pre = 0;
        let mut sum = 0;
        for x in nums2{
            if(x > 0 && pre > 0){
                sum = sum + x - pre;
                if(x == pre){
                    sum = 6;
                    break;
                }
            }
            if(x > 0){
                pre = x;
            }
            println!("{}", sum);
        }
        if(sum < 5){
            return true;
        }
        return false;
    }
}