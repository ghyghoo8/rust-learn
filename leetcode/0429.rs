/*
* 练手题：https://leetcode-cn.com/problems/first-unique-character-in-a-string/

* 附加题：https://leetcode-cn.com/problems/asteroid-collision/
*/
impl Solution {
    pub fn first_uniq_char(s: String) -> i32 {
        let _reChar:Vec<char> = s.chars().collect();
        let mut _idx:i32 = -1;
        for i in 0.._reChar.len(){
            let k = _reChar[i];
            if(s.find(k) == s.rfind(k)){
                _idx = i as i32;
                break;
            }
        }
        if(_idx > -1){
            return _idx;
        }
  
        return -1;
    }
}