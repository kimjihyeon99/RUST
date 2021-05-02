## Ciphers

### [Caesar](./caesar.rs)


<img src = https://user-images.githubusercontent.com/44187194/116805790-b44d2a00-ab63-11eb-81e2-319e2b42c1e3.png width=400 height=200>

- 암호학에서 다루는 간단한 치환 암호의 일종.

- 암호화 하고자 하는 내용을 알파벳별로 일정한 거리만큼 밀어서 다른 알파벳으로 치환하는 방식.

- 예) 3글자씩 밀기 'COME TO ROME' -> 'FRPH WR URPH'

- 카이사르 암호는 간단하여 일반인도 쉽게 사용할 수 있지만, 철자의 빈도와 자주 사용되는 단어와

  형태를 이용하면 쉽게 풀 수 있다는 단점이있다. 


### [Vigenere](./vigenere.rs)

<img src =https://user-images.githubusercontent.com/44187194/116806764-190b8300-ab6a-11eb-988e-5d15296c424b.png width=600 height=400>


- 이미지의 '비즈네르 표'는 원문 알파벳 아래에 26가지의 사이퍼 알파벳으로 나열됨.

- 사이퍼 알파벳은 한 줄 내려갈때마다 한 자씩 뒤로 이동, 1번 줄은 1칸 이동 카이사르 사이퍼 알파벳과 동일

- 예) 사이퍼 알파벳 4번 

  a-> E로 대체 , g -> K로 대체
  
- 키워드(key)는 수신자와 송신자가 아무 단어나 선택 가능

- 예) 키워드 : sky 


<img src =https://user-images.githubusercontent.com/44187194/116806836-a8189b00-ab6a-11eb-813b-1ea52bfd3e6e.png width=700 height=100>

- 같은 'o'에 대해서 'M','G','Y'세가지가 나온 것을 알 수 있다.

- 따라서 빈도 분석법으로 해독하지 못하게 된다. 
