use std::vec::Vec;

fn main() {
    // creat an array to be sorted
    let mut v = vec![1, 6, 4, 9, 10, 3, 2, 111, 222];


    let sorted = quick_sort(&mut v);
    println!("{:?}", sorted);

}

fn quick_sort(v: &mut Vec<i32>) -> Vec<i32> {
    let mut pivot: usize = v.len() - 1;
    let mut wall = 0;
    let mut target: usize = 0;

    for _ in 0..v.len()-1 {
        if v[target] < v[pivot] {
            wall += 1;
        }
        target += 1;

        for i in 1..pivot-1 {
            if v[target] < v[pivot] {
                let original = v[wall];
                v[wall] = v[target];
                v[target] = original;
                wall += 1;
            }
            target += 1;
        }

        // change the pivot to start the next iteration
        if v[wall] == v[pivot] {
            // the pivot is the largest
            pivot -= 1;
        } else {
            let original = v[wall];
            v[wall] = v[pivot];
            v[pivot] = original;
        }

        wall = 0;
        target = 0;
    }

    return v.clone();
}
