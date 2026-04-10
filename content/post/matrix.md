---
title: "线性代数期末预习"
date: 2024-11-12T00:00:00+08:00
categories:
  - 学习笔记
tags:
  - 线性代数
  - 数学
---

![](https://vip.helloimg.com/i/2024/04/24/6628cf348b9a0.jpg)

## 线性变换的定义

如果我们将线性变换看作是一种对于向量的线性变换关系，这种关系是广义的，可以涉及很多方面。但是，在线性代数层面而不是抽象代数层面，我们当然希望这种变换
是符合线性的，也就是说，我们希望一种变换$\boldsymbol{L}$对于向量$ \vec v_1 \vec v_2 $满足以下关系：

c \boldsymbol{L}\left(\boldsymbol{v}_1\right) = \boldsymbol{L}\left(\boldsymbol{v}_1\right)\boldsymbol{L}\left(\boldsymbol{v}_1\right) + \boldsymbol{L}\left(\boldsymbol{v}_2\right) = \boldsymbol{L}\left(\boldsymbol{v}_1 + \boldsymbol{v}_2\right) + \boldsymbol{L}\left(\boldsymbol{v}_2\right)
这样，我们就说这种变化是一种线性变换。

简化一下: 

\\L(\alpha\boldsymbol{v}_1+\beta\boldsymbol{v}_2)=\alpha L(\boldsymbol{v}_1)+\beta L(\boldsymbol{v}_2)
在数学上可以证明，对于一个n维向量$a$ 和m维向量 $b$来说，存在一个矩阵 $\boldsymbol{A}$使得$\boldsymbol{A}a= b$成立，这也被称为线性变化的矩阵化表述。

现在拓展到向量之外，如果一种函数或者任何抽象的代数形式，对于$f\left(x_1\right)\rightarrow f\left(x_2\right) $ 也满足上述
条件，那么，我们也说它是一种线性变换。

## 特征值（听说还叫本征值）的相关知识

特征值作为矩阵很重要的一个特征量，在整个矩阵学习过程中占据着很重要的位置。首先，在做题过程中，特征值的定义应该首先被考虑$ Ax = \lambda x $ ,$\lambda $ 是矩阵的特征值，$ x $是矩阵的每个特征向量。然后我们需要对于这个式子做恰当变形，如果涉及 $ A^T $ , $ A^{-1} $ 等情况 ，应该优先考虑在等式两端左乘或者右乘来达到变量代换。上述等式应当作为证明题和计算题的核心。

与此同时，特征值也存在着一些二级公式，如 $ P\left(\lambda\right) $ 应当对应于 $ P\left(A\right) $ 的特征值。这在某些题目中可能存在一些应用。
其次，特征值是否是半单的也决定着矩阵是否可对角化，矩阵对角化即为将矩阵通过适当的分解使得矩阵能够实现$ \boldsymbol{A} = SVS^{-1} $，其中V是 

$$ \begin{equation}<br/>    \begin{bmatrix}<br/>     a_{11} &   & &  \\<br/>      & a_{22} &  & \\<br/>      &   & a_{33}&\\<br/>      &   &  &a_{44}<br/>     \end{bmatrix}<br/>\end{equation}</p>
<p>$$ , $S$是经过 Gran-Summit 正交化的由各个特征向量组成的矩阵。
