/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where T: PartialOrd,
{
    if array.len() <= 1 {
        return ;
    }
    let mid = quick_partision(array);
    sort(&mut array[0..mid]);
    sort(&mut array[mid+1..]);
}

fn quick_partision<T>(arry: &mut [T]) -> usize
where T: PartialOrd,
{
    let mut low = 0;
    let high = arry.len() - 1;
    for j in low..high {
        if arry[j] < arry[high] {
            arry.swap(low, j);
            low += 1;
        }
    }
    arry.swap(low, high);
    low 
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}