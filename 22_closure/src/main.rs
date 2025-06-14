// fn main() {
//     // _closure();

//     // let double = |x: i32| x * 2;
//     // println!("{}", _apply_to_3(double));

//     // let x = 4;
//     // // // can't capture dynamic environment in a fn item
//     // // fn equal_to_x(z: i32) {
//     // //     z = x;
//     // // }
//     // let equal_to_x = |z| z == x;
//     // let y = 4;
//     // assert!(equal_to_x(y));

//     let mut num = 5;
//     // 按引用捕获
//     let add_num = |x: i32| x + num;
//     println!("{}", add_num(3)); // 输出：8
//     // 按可变引用捕获
//     let mut change_num = |x: i32| num += x;
//     change_num(5);
//     println!("{}", num); // 输出：10

//     let x = vec![1, 2, 3];
//     let equal_to_x = move |z| z = x;
//     // println!("can't use x here: {:?}", x);
//     let y = vec![1, 2, 3];
//     equal_to_x(y);
//     // println!("can't use y here: {:?}", y);
// }

// fn _closure() {
//     let c = |x| x;
//     let c_value = c(1);
//     println!("c_value: {c_value}");
// }

// fn _apply_to_3<F>(f: F) -> i32
// where
//     F: Fn(i32) -> i32,
// {
//     f(3)
// }

use std::collections::HashMap;
use std::hash::Hash;
// 课后作业
struct PageCache<K, V> {
    cache: HashMap<(K, V), String>,
}

impl<K, V> PageCache<K, V>
where
    K: Clone + Eq + Hash,
    V: Clone + Eq + Hash,
{
    fn new() -> Self {
        PageCache {
            cache: HashMap::new(),
        }
    }

    fn get_page<F>(&mut self, user_id: K, article_id: V, render: F) -> String
    where
        F: FnOnce(&K, &V) -> String, // FnOnce 确保闭包只能被调用一次
    {
        // 缓存键
        let cache_key = (user_id.clone(), article_id.clone());
        // 检查缓存是否存在
        if let Some(content) = self.cache.get(&cache_key) {
            // 缓存命中
            content.clone()
        } else {
            // 缓存未命中：调用渲染函数生成新内容
            let content = render(&user_id, &article_id);
            // 将新内容存入缓存（使用原始数据避免克隆开销）
            self.cache.insert((user_id, article_id), content.clone());
            content
        }
    }
}

fn main() {
    // 创建空缓存实例
    let mut page_cache = PageCache::new();
    // 第一次调用：执行渲染并缓存结果
    println!(
        "{}",
        page_cache.get_page("user1", 42, |user_id, article_id| {
            println!(
                "Rendering page for user {} and article {}",
                user_id, article_id
            );
            format!(
                "Rendered HTML content for user {} and article {}",
                user_id, article_id
            )
        })
    );
    // 第二次调用：相同用户和文章 - 直接返回缓存
    println!(
        "{}",
        page_cache.get_page("user1", 42, |_, _| {
            // 这个闭包在缓存命中时不会执行
            unreachable!("This should never be called when cache exists");
        })
    );
    // 不同用户：重新执行渲染
    println!(
        "{}",
        page_cache.get_page("user2", 42, |user_id, article_id| {
            println!(
                "Rendering page for user {} and article {}",
                user_id, article_id
            );
            format!(
                "Rendered HTML content for user {} and article {}",
                user_id, article_id
            )
        })
    );
}
