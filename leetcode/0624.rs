/*
* 练手题：合并有序数组 https://leetcode-cn.com/problems/merge-sorted-array/
* 附加题：合并账户 https://leetcode-cn.com/problems/accounts-merge/
*/
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut _n1Itr = 0;
        let mut len = m as usize;
        if(n==0){
            return;
        }
        if(m == 0){
            for i in 0..n{
                nums1[i as usize] = nums2[i as usize];
            }
            return;
        }
        for i in 0..n{
            let _i = i as usize;
            let mut _itr = _n1Itr;
            while _itr <= len {
                let _vi = _itr as usize;
                let _v1 = nums1[_vi];
                _itr += 1;
                _n1Itr = _itr;
                // println!("{:?}", _itr);
                if(_i < len) && (nums2[_i] < _v1){
                    len +=1;
                    nums1.splice(_vi.._vi, [nums2[_i]].iter().cloned());
                    break;
                } else if(_itr >= len){
                    len +=1;
                    nums1[_itr] = nums2[_i];
                    break;
                }
            }
        }
        let _len = m + n;
        nums1.truncate(_len as usize);
    }
}