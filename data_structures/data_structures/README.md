# 개념정리 

### 1. b-tree

 #### 📌 자식노드의 개수가 2개 이상인 드리. 차수가 홀수인지 짝수인지에 따라 알고리즘이 달라진다.
 
  - 노드의 데이터수가 n개라면 자식 노드의 개수는 n+1개이다. 
  - 노드의 데이터는 반드시정렬된 상태여야한다.
  - 노드의 자식노드의 데이터들은 노드 데이터를 기준으로 데이터보다 작은값은 
  
    왼쪽 서브 트리에 큰값들은 오른쪽 서브트리에 이루어져야한다.
  - Root 노드가 자식이 있다면 2개이상의 자식을 가져야한다. 
  - `Root` 노드를 제외한 모든 노드는 적어도 M/2개의 데이터를 갖고 있어야한다.
  - `Leaf` 노드로 모두 같은레벨에 존재해야한다.
  - 입력자료는 중복 불가이다.

#### 📌 탐색 
  
  root 노드 부터 시작해 하향식. 작은 값은 왼쪽 서브트리, 큰 값은 오른쪽 서브트리
  
#### 📌 삽입
  
  - 홀수 B-Tree
    
 <img src =https://user-images.githubusercontent.com/44187194/116566281-986a3e00-a941-11eb-86f5-fca7141fa009.png width="600" height="350">
   
  초기 삽입시에는 `root`노드를 생성
  
  1.데이터를 탐색해 해당하는 `Leaf`노드에 데이터를 삽입한다.
  
  2.`Leaf`노드데이터가 가득 차 있으면 노드를 분리한다.  
    
   -insert7에서 노드가 1,5,7로 가득차서, 중간값을 부모노드로 해서 트리를 구성
    
  3.분리한 서브트리가 b-tree조건에 맞지 않는다면 부모노드로 올라가며 merge한다.
  
   -insert12에서 서브트리로 분리하였으나 조건(`Leaf`노드가 같은 레벨이 아님)에 맞지않음. 
    
   -merge로 조건을 만족시킴.
    
#### 📌 삭제

  `Leaf`노드인 경우와 아닌경우로 나뉜다.

  1. `Leaf`노드인 경우
  
  예1)
 
  <img src=https://user-images.githubusercontent.com/44187194/116567506-a7052500-a942-11eb-89a3-0c76626cb611.png width="600" height="350">

  `Leaf`노드를 삭제하는 경우, 데이터를 삭제하여도 조건 충족함.

  예2)
  
   <img src=https://user-images.githubusercontent.com/44187194/116567559-b2f0e700-a942-11eb-92b8-f62942caeb38.png width="600" height="350">

  `Leaf`노드에서 1을 삭제하면 B-Tree 구조가 깨짐. 조건을 만족할때까지 merge 반복함.
  
  2. `Leaf`노드가 아닌 경우

  예1) 
  
  <img src= https://user-images.githubusercontent.com/44187194/116567844-f4819200-a942-11eb-80d3-4f1fa90f31a8.png width="600" height="350">

  예2)
  
  <img src= https://user-images.githubusercontent.com/44187194/116567988-167b1480-a943-11eb-85dd-2b25ff726dfb.png width="600" height="350">
