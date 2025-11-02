use std::primitive;

fn selection_sort(array : &mut Vec<i8>) -> Vec<i8>{
    for i in 0..array.len() - 1{
        let mut smallest = i;
        for j in (i + 1)..array.len(){
            if array[j] < array[smallest]{
                smallest = j;
            }
        }
        array.swap(smallest, i);
    }
    array.to_vec()
}


fn bubble_sort(array : & mut Vec<i8>) ->Vec<i8>{
    let mut sorted = true;
    for _ in 1..=array.len() - 1{
        sorted = true;
        for j in 0..=array.len() - 2{
            // println!("{}", j);
            if array[j] > array[j + 1]{
                array.swap(j, j + 1);
                sorted = false;
            }
        }
        if sorted{
            break;

        }
    }

    array.to_vec()

}

fn merge_sort(arr: &mut [i32]) -> Vec<i32>{
    if arr.len() > 1{
        let mid = arr.len()/2;
        merge_sort(&mut arr[..mid]);
        merge_sort(&mut arr[mid..]);
        merge(arr, mid);
    }
    arr.to_vec()
}

fn merge(arr:&mut [i32], mid:usize){
    let left = arr[..mid].to_vec();
    let right = arr[mid..].to_vec();

    let mut l = 0;
    let mut r = 0;

    for val in arr{
        if r == right.len() || (l < left.len() && left[l] < right[r]){
            *val = left[l];
            l += 1;
        }
        else{
            *val = right[r];
            r += 1;
        }
    }

}

fn quick_sort(arr: &mut [i32], start:usize, end:usize) -> Vec<i32>{
    if start < end {
        let part = partition(arr, start, end);
        quick_sort(arr, start, part - 1);
        quick_sort(arr, part + 1, end);
    }

    arr.to_vec()
}

fn partition(arr: &mut [i32], start: usize, end:usize) -> usize{
    let mut i = start;
    let pivot = end;

    for j in start..= end - 1{
        if arr[j] < arr[pivot]{
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, pivot);
    i
}


fn main() {
    let mut arr:Vec<i8> = vec![4,3,1,2];
    println!("Before sorting{:?}", arr);
    selection_sort(&mut arr);
    println!("after sorting{:?}", arr);


    let mut arr2:Vec<i8> = vec![4,3,1,2];
    println!("Before Bubble sorting{:?}", arr2);
    bubble_sort(&mut arr2);
    println!("after Bubble  sorting{:?}", arr2);


    let mut arr3:Vec<i32> = vec![4,7,3,5,1,2];
    println!("Before Bubble sorting{:?}", arr3);
    merge_sort(&mut arr3);
    println!("after Bubble  sorting{:?}", arr3);

    let mut array = vec![8,5,1,2,7,3,4];
    let len = array.len();
    quick_sort(&mut array, 0, len - 1);
    println!("Quick sort{:?}", array);
}
