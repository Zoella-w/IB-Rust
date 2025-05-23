// fn main() {
//     // _arr();

//     // let arr = [10, 20, 30, 40];
//     // println!("main before {:?}", arr); // 10 20 30 40
//     // _update(arr);
//     // println!("main after {:?}", arr); // 10 20 30 40

//     // let mut arr_mut = [10, 20, 30, 40];
//     // println!("main before {:?}", arr_mut); // 10 20 30 40
//     // _update_mut(&mut arr_mut);
//     // println!("main after {:?}", arr_mut); // 10 20 30 40

//     slice_show();
// }

// fn _arr() {
//     // // let arr: [i64;4] = [10, 20, 30, 40];
//     // let arr = [10, 20, 30, 40];
//     // println!("{:?}", arr);

//     // let arr1: [i32; 4] = [10; 4]; // [10, 10, 10, 10]
//     // println!("{:?}", arr1);

//     // // 长度
//     // println!("arr len is {:?}", arr.len());

//     // // 遍历
//     // // for index in 0..4 {
//     // //     println!("{:?}", arr[index]);
//     // // }
//     // // arr.iter().for_each(|value| println!("{:?}", value));
//     // for value in arr.iter() {
//     //     println!("{:?}", value);
//     // }

//     // 不可变数组&可变数组
//     // let arr: [i32; 4] = [10, 20, 30, 40];
//     // arr[0] = 0; // error
//     let mut arr_mut: [i32; 4] = [10, 20, 30, 40];
//     println!("{:?}", arr_mut);
//     arr_mut[0] = 0;
//     println!("{:?}", arr_mut);
// }

// // 数组作为值传递
// fn _update(mut arr: [i32; 4]) {
//     println!("update before {:?}", arr); // 10 20 30 40
//     for i in 0..4 {
//         arr[i] = 0;
//     }
//     println!("update before {:?}", arr); // 10 20 30 40
// }

// // 数组作为引用传递
// fn _update_mut(arr: &mut [i32; 4]) {
//     println!("update before {:?}", arr); // 10 20 30 40
//     for i in 0..4 {
//         arr[i] = 0;
//     }
//     println!("update before {:?}", arr); // 10 20 30 40
// }

// fn slice_show() {
//     // let a = [1, 2, 3, 4, 5];
//     // let slice = &a[1..3];
//     // println!("{:?}", slice);
//     // let mut_slice = &mut a[1..3];
//     // println!("{:?}", mut_slice);

//     let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
//     println!("{:?}", arr.len());

//     // // let s = &arr[0];
//     // let s: &[i32] = &[];
//     // println!("is_empty is {:?}", s.is_empty());

//     // for i in arr.windows(3) {
//     //     println!("windows is: {:?}", i);
//     // }

//     // println!("{}", arr.starts_with(&[10])); // false
//     println!("{}", arr.starts_with(&[1, 2, 3])); // true
// }

// 课后习题：
// 给定一个整数数组 nums，返回一个数组 answer ，使得 answer[i] 等于 nums 除之外 nums[i] 的所有元素的乘积。
// 任何前缀或后缀的乘积 nums 都保证适合 32 位整数。
// 您必须编写一个能够及时运行 O(n) 且无需使用除法运算的算法。

// 示例 1：
// 输入： nums = [1,2,3,4]
// 输出： [24,12,8,6]
// 示例 2：
// 输入： nums = [-1,1,0,-3,3]
// 输出： [0,0,9,0,0]

// 限制：
// 2 <= nums.length <= 105
// -30 <= nums[i] <= 30
// 任何前缀或后缀的乘积 nums 都保证适合 32 位整数。

// 进阶： 你能以 O(1) 额外空间复杂度解决这个问题吗？（输出数组不算作空间复杂度分析的额外空间。）

fn main() {
    let nums = [1, 2, 3, 4];
    let answer = get_answer(nums.to_vec());
    println!("answer is {:?}", answer); // [24, 12, 8, 6]
}

fn get_answer(nums: Vec<i32>) -> Vec<i32> {
    let n = nums.len();
    let mut answer = vec![1; n];

    // 计算前缀乘积：answer[i]
    for i in 1..n {
        // nums[1, n-1] 的前缀乘积（nums[0] 没有前缀）
        answer[i] = answer[i - 1] * nums[i - 1];
    }

    // 计算后缀乘积，并更新 answer[i]
    let mut right = 1; // right 表示当前元素右侧所有元素的乘积
    for i in (0..n).rev() {
        // 从 nums[n-1..0] 的乘积
        answer[i] *= right; // 前缀乘积 * 后缀乘积
        right *= nums[i]; // 更新后缀乘积
    }

    answer
}
