//linked list 구현
//출력을 위한 trait
use std::fmt::{self, Display, Formatter};
//nonnull을 쓰는 이유? 느리게 할당되는 유형을 초기화하는데 유용
use std::ptr::NonNull;

//node 구조체
struct  Node<T>{
    val:T,
    next: Option<NonNull<Node<T>>>,
    prev: Option<NonNull<Node<T>>>,
}

impl <T> Node<T>{
    fn new(t:T) -> Node<T>{
        Node{
            val:t,
            //rust에는 null이 없음.
            prev:None,
            next:None,
        }
    }
}


pub struct LinkedList<T>{
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

//여러 타입으로 LinkedList 생성 가능
impl<T> Default for LinkedList<T>{
    fn default() ->Self{
        Self::new()
    }
}

//초기화
impl<T> LinkedList<T>{
    pub fn new()->Self{
        Self{
            length:0,
            start:None,
            end:None,
        }
    }

    pub fn add(&mut self, obj: T){//뒤에 원소 추가
        //box로  힙 데이터를 참조 할 수 있음
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        node.prev = self.end;
        //unsafe : 안전하지 않은 러스트로 전환
        //new_unchecked : unsafe로서 사용자가 그 데이터의 재배치나 무효화를 하지 않을 책임을 짐
        let node_ptr = Some(unsafe{NonNull::new_unchecked(Box::into_raw(node))});
        match self.end {
            //마지막 원소가 없으면, 시작위치에 넣기
            None =>self.start = node_ptr,
            //마지막 원소가 있으면, 마지막 원소의 다음에 넣기
            Some(end_ptr) => unsafe{(*end_ptr.as_ptr()).next = node_ptr},
        }
        //마지막 원소에 업데이트
        self.end = node_ptr;
        //길이 업데이트
        self.length +=1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T>{
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node:Option<NonNull<Node<T>>>, index: i32) ->Option<&T>{
        match node{
            None =>None,
            Some(next_ptr) => match index{
                //인덱스가 0이면 첫번째 원소 반환
                0=> Some(unsafe{ &(*next_ptr.as_ptr()).val}),
                //인덱스가 1이상이면 인덱스 만큼 get_ith_node호출
                _=> self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
}
//링크드리스트 출력을 위한 메소드
impl<T> Display for LinkedList<T>
    where
        T:Display,
{
    fn fmt(&self, f:&mut Formatter) ->fmt::Result{
        match self.start{
            //start의 노드 출력
            Some(node) => write!(f,"{}", unsafe{node.as_ref()}),
            None=>Ok(()),
        }
    }
}
//노드 출력을 위한 메소드
impl<T> Display for Node<T>
    where
        T:Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            //현재 노드의 원소와 next가 가리키는 노드 출력
            Some(node) => write!(f,"{}, {}",self.val, unsafe{node.as_ref()}),
            //가리키는 다음 노드가 없다면 현재 노드의 원소만 반환
            None=>write!(f,"{}",self.val),
        }
    }
}

#[cfg(test)]
mod tests{
    use super::LinkedList;

    #[test]
    fn create_numeric_list(){
        //링크드리스트 생성
        let mut list = LinkedList::<i32>::new();
        //링크드리스트 원소 추가
        list.add(1);
        list.add(2);
        list.add(3);
        //링크드리스트 출력
        println!("Linked List is {}",list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn get_by_index_in_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        println!("Linked List is {}", list);
        let retrived_item = list.get(0);
        //원소가 잘 가져와졌는지 검사
        assert!(retrived_item.is_some());
        //가져온 원소가 1이 맞는지 검사
        assert_eq!(2 as i32, *retrived_item.unwrap());
    }


}