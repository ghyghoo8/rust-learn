/*
* 练手题：[检测大写字母](https://leetcode-cn.com/problems/detect-capital/)
* 附加题：[mini-parser](https://leetcode-cn.com/problems/mini-parser/)
*/
impl Solution {
    pub fn detect_capital_use(word: String) -> bool {
        let str = word.to_string();
        let strVec: Vec<char> = word.chars().collect();
        let mut upNum = 0;
        for i in str.chars() {
            if(i.to_string() == i.to_string().to_uppercase()){
                upNum = upNum + 1 ;
            }
        }
        if(strVec.len() == upNum) || (upNum == 0){
            return true;
        }
        let firstStr = strVec[0].to_string();
        if(firstStr == firstStr.to_uppercase()) && (upNum == 1){
            return true;
        }
 
        return false;
    }
}