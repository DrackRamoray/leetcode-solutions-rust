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

#[cfg(test)]
mod tests {
    use super::Twitter;

    #[test]
    fn it_works() {
        let mut twitter = Twitter::new();
        twitter.post_tweet(1, 5);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
        twitter.follow(1, 2);
        twitter.post_tweet(2, 6);
        assert_eq!(twitter.get_news_feed(1), vec![6, 5]);
        twitter.unfollow(1, 2);
        assert_eq!(twitter.get_news_feed(1), vec![5]);
    }
}
