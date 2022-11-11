### [391. 完美矩形](https://leetcode.cn/problems/perfect-rectangle/)
给你一个数组 rectangles ，其中 rectangles[i] = [x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub>] 表示一个坐标轴平行的矩形。这个矩形的左下顶点是 (x<sub>i</sub>, y<sub>i</sub>) ，右上顶点是 (a<sub>i</sub>, b<sub>i</sub>) 。

如果所有矩形一起精确覆盖了某个矩形区域，则返回 true ；否则，返回 false 。


##### 示例 1：
![img.png](img.png)
```
输入：rectangles = [[1,1,3,3],[3,1,4,2],[3,2,4,4],[1,3,2,4],[2,3,3,4]]
输出：true
解释：5 个矩形一起可以精确地覆盖一个矩形区域。
```

##### 示例 2：
![img_1.png](img_1.png)
```
输入：rectangles = [[1,1,2,3],[1,3,2,4],[3,1,4,2],[3,2,4,4]]
输出：false
解释：两个矩形之间有间隔，无法覆盖成一个矩形。
```

##### 示例 3：
![img_2.png](img_2.png)
```
输入：rectangles = [[1,1,3,3],[3,1,4,2],[1,3,2,4],[2,2,4,4]]
输出：false
解释：因为中间有相交区域，虽然形成了矩形，但不是精确覆盖。
```

##### 提示：
- 1 <= rectangles.length <= 2 * 10<sup>4</sup>
- rectangles[i].length == 4
- -10<sup>5</sup> <= x<sub>i</sub>, y<sub>i</sub>, a<sub>i</sub>, b<sub>i</sub> <= 10<sup>5</sup>

##### 题解：
```rust

```
