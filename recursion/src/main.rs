fn fact(num: i32) ->i32{
    if num > 1{
        return num * fact(num -1)
    }
    else{
        return 1
    }
}

fn fib(num: i32) -> i32{
    if num == 0 {
        return 0
    }
    if num == 1{
        return 1
    }

    let n1 = fib(num -1);
    let n2 = fib(num -2);

    return n1 + n2;
}

fn palindrome(array: &Vec<i32>, start: usize, end:usize) ->bool{
    if start >= end{
        return true
    }

    if array[start] == array[end]{
        return palindrome(array, start + 1, end - 1)
    }
    else{
        return false
    }
}

//tower of hanoi
fn toh(num:i32) -> i32{
    if num == 0 {
        return 0;
    }

    else{
        return toh(num - 1) + 1 + toh(num - 1);
    }

}

//sum of triangles
fn sumta(array: &Vec<i32>){
    
    if array.len() < 1{
        return;
    }
    let mut temp: Vec<i32> = vec![];
    for x in 0..array.len()-1{
        temp.push(array[x] + array[x+1]);
    }

    sumta(&temp);
    println!("{:?}", array);


}

fn main() {
    println!("{}", fact(5));

    println!("{}", fib(15));

    let array = vec![1,2,3,4];
    println!("{}", palindrome(&array, 0, array.len() - 1));

    let array2 = vec![1, 2 ,3, 4, 3, 2, 1];
    println!("{}", palindrome(&array2, 0, array2.len() - 1));


    println!("{}", toh(3));

    let nums = vec![1, 2, 3, 4, 5];
    sumta(&nums);
    

}
