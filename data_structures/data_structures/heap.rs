//Heap data structure

use std::cmp::Ord; //순서대로 정리하기 위한것
use std::default::Default;

pub struct Heap<T>
where
    T:Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}


impl<T> Heap<T>
where
    T:Default,
{
    //생성자
    pub fn new(comparator: fn(&T,&T)->bool)->Self{
        Self{
            count:0,
            //vec은 모든 같은 종류 타입을 가지고 있다.
            items:  vec![T::default()],
            comparator,
        }
    }
    //길이 구하는 함수
    pub fn len(&self)->usize{
        self.count //사이즈 반환
    }

    //비었는지 확인하는 함수
    pub fn is_empty(&self)->bool{
        self.count == 0 //사이즈가 0이면 true 반환
    }

    //item을 추가하는 함수
    pub fn add(&mut self, value: T){
        self.count += 1; // 사이즈 하나 증가
        self.items.push(value);

        //heapify up: 부모 노드와 값을 비교하면서 정렬
        let mut idx = self.count;

        while self.parent_idx(idx) > 0 {
            //부모의 인덱스 값을 pdx에 저장
            let pdx = self.parent_idx(idx);
            //부모와 자식 노드 비교 후 자리 바꿈
            if(self.comparator)(&self.items[idx], &self.items[pdx]){
                self.items.swap(idx, pdx);
            }
            //인덱스 값도 업데이트
            idx = pdx;
        }
    }

    //parent의 인덱스를 알아내는 함수
    fn parent_idx(&self, idx: usize)-> usize{
        idx / 2
    }

    //나의 왼쪽 자식의 인덱스가 트리 사이즈보다 크면 false 반환 -> iterator 를 위한것
    fn children_parent(&self, idx:usize)->bool{
        self.left_child_idx(idx) <= self.count
    }

    //왼쪽 자식의 인덱스 반환하는 함수
    fn left_child_idx(&self, idx:usize)->usize{
        idx * 2
    }
    //오른쪽쪽 자식의 인덱스 반환는 함수
    fn right_child_idx(&self, idx:usize)->usize{
        self.left_child_idx(idx) + 1
    }

    //가장 작은 값을 가지는 인덱스를 반환
    fn smallest_child_idx(&self, idx:usize)->usize{
        //왼쪽자식만 있는 경우
        if self.right_child_idx(idx) > self.count {
            self.left_child_idx(idx)
        //왼쪽 오른쪽 둘다 있는 경우
        }else {
            let ldx = self.left_child_idx(idx);
            let rdx = self.right_child_idx(idx);
            if(self.comparator)(&self.items[ldx], &self.items[rdx]){
                ldx
            }else{
                rdx
            }
        }
     }

}

//create heap
impl<T> Heap<T>
where
    T:Default+Ord,
{
    //Minheap 생성
    pub fn new_min()->Self{
        Self::new(|a, b| a<b)
    }

    //Maxheap 생성
    pub fn new_max()->Self{
        Self::new(|a,b| a>b)
    }
}

//Iterater 역할
impl<T> Iterator for Heap<T>
where
    T:Default,
{
    type Item = T;

    fn next(&mut self)->Option<T>{
        let next = if self.count==0{
            None
        }else{
            //swap_remove() 하나씩 없애면서 탐색
            //1번째 인덱스의 값을 없에면서 보기, 인덱스를 하나만 사용 가능
            let next =self.items.swap_remove(1);
            Some(next)
        };
        self.count -=1;

        if self.count>0{
            //heapify down
            let mut idx =1;
            while self.children_parent(idx) {
                let cdx = self.smallest_child_idx(idx);
                if !(self.comparator)(&self.items[idx], &self.items[cdx]){
                    self.items.swap(idx, cdx);
                }
                idx = cdx;
            }
        }
        next
    }

}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default+Ord,
    {
        Heap::new(|a,b| a<b)
    }
}

pub struct MaxHeap;

impl MaxHeap{
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() ->Heap<T>
    where
        T:Default+Ord,
    {
        Heap::new(|a,b| a>b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }

    struct Point(/* x */ i32, /* y */ i32);
    impl Default for Point {
        fn default() -> Self {
            Self(0, 0)
        }
    }

    #[test]
    fn test_key_heap() {
        let mut heap: Heap<Point> = Heap::new(|a, b| a.0 < b.0);
        heap.add(Point(1, 5));
        heap.add(Point(3, 10));
        heap.add(Point(-2, 4));
        assert_eq!(heap.len(), 3);
        assert_eq!(heap.next().unwrap().0, -2);
        assert_eq!(heap.next().unwrap().0, 1);
        heap.add(Point(50, 34));
        assert_eq!(heap.next().unwrap().0, 3);
    }
}
