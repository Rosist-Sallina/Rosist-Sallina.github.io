---
title: "面向计算机的科学的离散数学（期中）"
date: 2024-11-12T00:00:00+08:00
categories:
  - 学习笔记
tags:
  - 离散数学
  - 数学
---

test_1
![](https://vip.helloimg.com/i/2024/04/23/6627c8e7ef60d.jpg)

##### tips:本文逻辑和顺序可能混乱，因为作者是根据题目进行知识点的复习

### 1.在集合A上等价关系个数的计算

等价关系与划分一一对应，非空集合A上的等价关系个数等于A的划分个数，即等价关系个数等于划分个数。
转换为数学模型：即将n个小球放入m个相同的盒子中，并且不能出现空盒，不同的放球方式代表着不同的划分方式
再转换为第二类斯特林数：$S(n,m)$，代表以上方法

#### 第二类斯特林数计算方法

- 递推公式：$S(n,m) = S(n-1,m-1) + mS(n-1,m)  \rightarrow $ 注意到等价于背包问题

- $ S(n , 0) = 0 , S(n , 1) = 1 , S(n , 2) = 2^{n-1}-1 , S(n , n-1) = C(n , 2) , S(n , n ) = 1$

#### 4元集等价关系计算

- 方法：利用第二类斯特林数计算

- 过程：N = S(4,1) + S(4,2) + S(4,3) + S(4,4) = 1=7+6+1 = 15![](https://vip.helloimg.com/i/2024/04/23/6627c8e813295.jpg)
#### 6元集等价关系计算

- 方法：利用第二类斯特林数计算

##### Tips：列出从1-10的BEll数，为 1 , 2 , 5 , 15 , 52 , 203 , 877 , 4140 , 21147 , 115975

### 4. 证明无限集和实数集等势

方法：构造函数形如

f(x) = x, \quad x \in \mathbb{R} \setminus (\mathbb{Q} \cup \mathbb{Q}) + \sqrt{2}
f(x) = g(x), \quad x \in (\mathbb{Q} \cup \mathbb{Q}) + \sqrt{2}
其中 $ g(x) $ 为 $ \mathbb{Q} \cup \mathbb{Q} + \sqrt{2} \Rightarrow \mathbb{Q} + \sqrt{2} $ 的映射。由于前者为可数集，并且可数集等于可数集，可数集与可数集之间一定存在一一映射，$ g(x) $ 存在。

定义域为 $ \mathbb{R} $，值域为

\mathbb{R} \setminus (\mathbb{Q} + \mathbb{Q}) + \mathbb{Q} + \mathbb{Q} + \sqrt{2} = \mathbb{R} / \mathbb{Q}
证明了该函数为从 $ \mathbb{R} $ 到 $ \mathbb{R} / \mathbb{Q} $ 的一一映射，即等势得证。

### 4.证明无限集和实数集等势

方法：构造函数形如

f(x) = x , x \in R/(Q \cup Q) + \sqrt{2}f(x)  = g(x)  , x \in (Q\cup Q) + \sqrt{2}
其中g(x)为 Q \cup Q + \sqrt{2} \Rightarrow Q + \sqrt{2} 的映射。由于前者为可数集并可数集等于可数集，可数集与可数集之间一定存在一一映射，g(x)存在。
定义域为R，值域为R/(Q + Q) + Q + Q + \sqrt{2} = R / Q
证明了该函数为从$ R $到$ R / Q $的一一映射，即等势得证

转自知乎：[https://zhuanlan.zhihu.com/p/385762588](https://zhuanlan.zhihu.com/p/385762588)

### 5.一些基础（虽然可能不值得写）

- P\rightarrow Q = \neg P \vee Q

- P \leftrightarrow Q = (\neg P \vee Q) \wedge (\neg Q \vee P)

- 证明A \rightarrow B 为真 \Leftrightarrow 证明 A \wedge \neg B 为矛盾式

- 空关系是一种等价关系，具有传递性，自反性，对称性

- P \rightarrow (Q \rightarrow R ) \neq ( P \rightarrow Q)\rightarrow R  易证

- 一个偏序关系的极大元需要属于这个集合，且极大元的定义是没有比它更大的元素

- 上确界的定义是每个元素都小于等于上确界，且不存在小于上确界的元素成为关系的上界![](https://vip.helloimg.com/i/2024/04/23/6627c8e8e8ca5.png)

### 6.根据无穷公理构造的自然数
1 = \{0\}2 = \{0, 1\}
以此类推
我们可以得到

\cap 1(2, 3 , 4...) = 0\cup n = n - 1
### 8.考虑各种闭包对于关系的破坏

显然自反性(r)，对称性(s)不会因为关系的扩张而受到影响，而对称性不一定，因此，我们需要考虑对这种情况谨慎处理

![](https://vip.helloimg.com/i/2024/04/23/6627c8e88e5bf.jpg)

### 9.求取主析取范式和主合取范式的一些方法

求取主析取范式基本策略：向式子后面补充$ \vee (p \wedge \neg p)$ ,变成一个一坨巨大的东西之后慢慢利用分配率等方式进行化简
求取主合取范式：利用主析取范式和主合取范式之间的关系可以很快求解，大概为找到剩余的项，然后分别用$ 2^n -1 $ 减掉即可
