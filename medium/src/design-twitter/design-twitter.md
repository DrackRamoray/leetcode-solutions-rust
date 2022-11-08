### [355. 设计推特](https://leetcode.cn/problems/design-twitter/)
设计一个简化版的推特(Twitter)，可以让用户实现发送推文，关注/取消关注其他用户，能够看见关注人（包括自己）的最近 10 条推文。

实现 Twitter 类：

- Twitter() 初始化简易版推特对象
- void postTweet(int userId, int tweetId) 根据给定的 tweetId 和 userId 创建一条新推文。每次调用此函数都会使用一个不同的 tweetId 。
- List<Integer> getNewsFeed(int userId) 检索当前用户新闻推送中最近  10 条推文的 ID 。新闻推送中的每一项都必须是由用户关注的人或者是用户自己发布的推文。推文必须 按照时间顺序由最近到最远排序 。
- void follow(int followerId, int followeeId) ID 为 followerId 的用户开始关注 ID 为 followeeId 的用户。
- void unfollow(int followerId, int followeeId) ID 为 followerId 的用户不再关注 ID 为 followeeId 的用户。


##### 示例：
```
输入
["Twitter", "postTweet", "getNewsFeed", "follow", "postTweet", "getNewsFeed", "unfollow", "getNewsFeed"]
[[], [1, 5], [1], [1, 2], [2, 6], [1], [1, 2], [1]]
输出
[null, null, [5], null, null, [6, 5], null, [5]]

解释
Twitter twitter = new Twitter();
twitter.postTweet(1, 5); // 用户 1 发送了一条新推文 (用户 id = 1, 推文 id = 5)
twitter.getNewsFeed(1);  // 用户 1 的获取推文应当返回一个列表，其中包含一个 id 为 5 的推文
twitter.follow(1, 2);    // 用户 1 关注了用户 2
twitter.postTweet(2, 6); // 用户 2 发送了一个新推文 (推文 id = 6)
twitter.getNewsFeed(1);  // 用户 1 的获取推文应当返回一个列表，其中包含两个推文，id 分别为 -> [6, 5] 。推文 id 6 应当在推文 id 5 之前，因为它是在 5 之后发送的
twitter.unfollow(1, 2);  // 用户 1 取消关注了用户 2
twitter.getNewsFeed(1);  // 用户 1 获取推文应当返回一个列表，其中包含一个 id 为 5 的推文。因为用户 1 已经不再关注用户 2
```

##### 提示：
- 1 <= userId, followerId, followeeId <= 500
- 0 <= tweetId <= 10<sup>4</sup>
- 所有推特的 ID 都互不相同
- postTweet、getNewsFeed、follow 和 unfollow 方法最多调用 3 * 10<sup>4</sup> 次

##### 题解：
```rust
use std::collections::HashSet;
use std::collections::HashMap;

struct User {
    followers: HashSet<i32>,
    twitters: Vec<i32>
}

impl User {
    fn new() -> Self {
        User{
            followers: HashSet::new(),
            twitters: Vec::new(),
        }
    }

    fn follow(&mut self, user_id: i32) {
        self.followers.insert(user_id);
    }

    fn unfollow(&mut self, user_id: i32) {
        self.followers.remove(&user_id);
    }

    fn get_followers(&self) -> Vec<i32> {
        self.followers.clone().into_iter().collect::<Vec<i32>>()
    }

    fn post_twitter(&mut self, tweet_id: i32) {
        self.twitters.insert(0, tweet_id);
    }

    fn del_twitter(&mut self) {
        self.twitters.pop();
    }

    fn get_twitters(&self) -> Vec<i32> {
        self.twitters.clone()
    }

    fn get_twitter_size(&self) -> usize {
        self.twitters.len()
    }
}

pub struct Twitter {
    size: usize,
    time: i32,
    tweet_time: HashMap<i32, i32>,
    users: HashMap<i32, User>
}

impl Twitter {

    fn new() -> Self {
        Twitter{
            size: 10_usize,
            time: 0,
            tweet_time: HashMap::new(),
            users: HashMap::new()
        }
    }

    fn add_user(&mut self, user_id: i32) {
        self.users.insert(user_id, User::new());
    }

    fn post_tweet(&mut self, user_id: i32, tweet_id: i32) {
        if !self.users.contains_key(&user_id) {
            self.add_user(user_id);
        }

        let user = self.users.get_mut(&user_id).unwrap();

        if user.get_twitter_size() > self.size {
            user.del_twitter();
        }

        user.post_twitter(tweet_id);
        self.time += 1;
        self.tweet_time.insert(tweet_id, self.time);
    }

    fn get_news_feed(&self, user_id: i32) -> Vec<i32> {
        if let Some(user) = self.users.get(&user_id) {
            let mut ans = Vec::new();
            let tweets = user.get_twitters();

            tweets.into_iter().for_each(|tweet_id| ans.push(tweet_id));

            user
                .get_followers()
                .into_iter()
                .for_each(|followee_id| {
                    if let Some(user) = self.users.get(&followee_id) {
                        if user_id != followee_id {
                            let tweets = user.get_twitters();
                            tweets.into_iter().for_each(|tweet_id| ans.push(tweet_id));
                        }
                    }
                });

            ans.sort_by(|a, b| self.tweet_time.get(b).unwrap().cmp(self.tweet_time.get(a).unwrap()));

            if ans.len() > self.size {
                ans[0..(self.size)].to_vec()
            } else {
                ans
            }
        } else {
            Vec::new()
        }
    }

    fn follow(&mut self, follower_id: i32, followee_id: i32) {
        if !self.users.contains_key(&follower_id) {
            self.add_user(follower_id);
        }

        if !self.users.contains_key(&followee_id) {
            self.add_user(followee_id);
        }

        if let Some(user) = self.users.get_mut(&follower_id) {
            user.follow(followee_id);
        }
    }

    fn unfollow(&mut self, follower_id: i32, followee_id: i32) {
        if let Some(user) = self.users.get_mut(&follower_id) {
            user.unfollow(followee_id);
        }
    }
}
```
