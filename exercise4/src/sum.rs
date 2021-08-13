/*
实现一个函数，为u32类型的整数集合求和，参数类型为 &[u32]，返回类型为Option，溢出时返回None，可以上传代码片段，或者代码的链接；
*/

fn sum(num_list: &[u32]) -> Option<u32> {
    let mut total:u32 = 0;

    for i in num_list.iter() {
        let res = total.checked_add(*i);
        if res == None {
            println!("OVERFLOW! at {}", i);
            return None;
        }
        total = res.unwrap();
    }
    Some(total)
}

pub fn problem2(){
    let x:[u32;7] = [1,2,3,4,429496729,6,7];
    let total = sum(&x);
 
    println!("Total: {}", total.unwrap())
}