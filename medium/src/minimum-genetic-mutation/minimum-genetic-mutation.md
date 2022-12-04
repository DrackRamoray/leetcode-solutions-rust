### [433. 最小基因变化](https://leetcode.cn/problems/minimum-genetic-mutation/)
基因序列可以表示为一条由 8 个字符组成的字符串，其中每个字符都是 'A'、'C'、'G' 和 'T' 之一。

假设我们需要调查从基因序列 start 变为 end 所发生的基因变化。一次基因变化就意味着这个基因序列中的一个字符发生了变化。

例如，"AACCGGTT" --> "AACCGGTA" 就是一次基因变化。
另有一个基因库 bank 记录了所有有效的基因变化，只有基因库中的基因才是有效的基因序列。（变化后的基因必须位于基因库 bank 中）

给你两个基因序列 start 和 end ，以及一个基因库 bank ，请你找出并返回能够使 start 变化为 end 所需的最少变化次数。如果无法完成此基因变化，返回 -1 。

注意：起始基因序列 start 默认是有效的，但是它并不一定会出现在基因库中。

##### 示例 1：
````
输入：start = "AACCGGTT", end = "AACCGGTA", bank = ["AACCGGTA"]
输出：1
````

##### 示例 2：
````
输入：start = "AACCGGTT", end = "AAACGGTA", bank = ["AACCGGTA","AACCGCTA","AAACGGTA"]
输出：2
````

##### 示例 3：
````
输入：start = "AAAAACCC", end = "AACCCCCC", bank = ["AAAACCCC","AAACCCCC","AACCCCCC"]
输出：3
````

##### 提示：
- start.length == 8
- end.length == 8
- 0 <= bank.length <= 10
- bank[i].length == 8
- start、end 和 bank[i] 仅由字符 ['A', 'C', 'G', 'T'] 组成

##### 题解：
````rust
use std::collections::HashSet;
use std::collections::VecDeque;

impl Solution {
    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        if start == end {
            return 0;
        }

        let gene = vec!["A", "C", "G", "T"];
        let mut step = 1;
        let mut cnt = HashSet::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        queue.push_back(start.clone());
        visited.insert(start);

        for w in bank.iter() {
            cnt.insert(w);
        }

        while queue.len() > 0 {
            let len = queue.len();
            for i in 0..len {
                let item = queue.pop_front().unwrap();

                for j in 0..8 {
                    for g in gene.iter() {
                        if g != &item.get(j..=j).unwrap() {
                            let s = item.get(0..j).unwrap_or("").to_string() + (*g) + item.get(j+1..).unwrap_or("");
                            if !visited.contains(&s) && cnt.contains(&s) {
                                if s.eq(&end) {
                                    return step;
                                }

                                queue.push_back(s.clone());
                                visited.insert(s);
                            }
                        }
                    }
                }
            }
            step += 1;
        }

        -1
    }
}
````
