// https://leetcode-cn.com/problems/get-watched-videos-by-your-friends/

use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn watched_videos_by_friends(
    watched_videos: Vec<Vec<String>>,
    friends: Vec<Vec<i32>>,
    id: i32,
    level: i32,
) -> Vec<String> {
    let level_friends = get_friends(friends, id, level);
    let mut videos = get_videos(watched_videos, level_friends); // 带频率好排序
    videos.sort_by(|a, b| match a.1.cmp(&b.1) {
        Ordering::Equal => a.0.cmp(&b.0),
        other => other,
    });

    videos.into_iter().map(|e| e.0).collect()
}

fn get_friends(friends: Vec<Vec<i32>>, id: i32, level: i32) -> Vec<i32> {
    let mut users = VecDeque::new();
    users.push_back(id);
    let mut visited_users: Vec<bool> = vec![false; friends.len()];
    visited_users[id as usize] = true;
    for _i in 0..level {
        let span = users.len();
        for _j in 0..span {
            let user_id = users.pop_front().unwrap();
            let user_friends: &Vec<i32> = &friends[user_id as usize];
            for f in user_friends {
                if !visited_users[(*f) as usize] {
                    users.push_back(*f);
                    visited_users[(*f) as usize] = true;
                }
            }
        }
    }

    let mut ans = vec![];
    for u in users {
        ans.push(u);
    }
    ans
}

fn get_videos(watched_videos: Vec<Vec<String>>, friends: Vec<i32>) -> Vec<(String, i32)> {
    let mut m: HashMap<String, i32> = HashMap::new();
    for friend in friends {
        let user_videos = &watched_videos[friend as usize];
        for video in user_videos {
            let counter = m.entry(video.clone()).or_insert(0);
            *counter += 1;
        }
    }
    let mut ans = vec![];
    for (k, v) in m {
        ans.push((k, v));
    }
    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1311() {
        let ans1 = watched_videos_by_friends(
            vec![
                vec![String::from("A"), String::from("B")],
                vec![String::from("C")],
                vec![String::from("B"), String::from("C")],
                vec![String::from("D")],
            ],
            vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]],
            0,
            1,
        );
        assert_eq!(vec![String::from("B"), String::from("C")], ans1);
    }

    #[test]
    fn test_get_friends() {
        let friends = vec![vec![1, 2], vec![0, 3], vec![0, 3], vec![1, 2]];
        let k_friends1 = get_friends(friends.clone(), 0, 1);
        assert_eq!(vec![1, 2], k_friends1);

        let k_friends2 = get_friends(friends, 0, 2);
        assert_eq!(vec![3], k_friends2);
    }

    #[test]
    fn test_get_videos() {
        let watched_videos = vec![
            vec![String::from("A"), String::from("B")],
            vec![String::from("C")],
            vec![String::from("B"), String::from("C")],
            vec![String::from("D")],
        ];
        let friends = vec![1, 2];
        let mut videos1 = get_videos(watched_videos, friends);
        videos1.sort_by(|a, b| a.1.cmp(&b.1));

        assert_eq!(
            vec![(String::from("B"), 1), (String::from("C"), 2)],
            videos1
        );
    }
}
