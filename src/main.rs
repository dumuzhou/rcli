fn main() {
    let mut x = 10;

    // 创建一个裸指针指向 x
    let raw_ptr: *const i32 = &x;

    unsafe {
        // 使用 unsafe 块解引用裸指针
        println!("不可变指针地址:{:p}", raw_ptr);
        println!("不可变指针解引用:{}", *raw_ptr);
    }

    // 使用可变的裸指针
    let mut y = 20;
    let mut raw_mut_ptr: *mut i32 = &mut y;

    unsafe {
        let raw_ptr: *const i32 = &y;
        println!("不可变指针地址:{:p}", raw_ptr);
        println!("可变指针地址: {:p}", raw_mut_ptr);
        println!("可变指针解引用: {}", *raw_mut_ptr);
        // 修改通过裸指针指向的变量的值
        *raw_mut_ptr = 30;
        println!("可变指针地址: {:p}", raw_mut_ptr);
        println!("可变指针解引用: {}", *raw_mut_ptr);
    }
}
