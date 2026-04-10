---
math: true
title: "矩阵的最小二乘法和投影矩阵"
date: 2024-11-12T00:00:00+08:00
categories:
  - 学习笔记
tags:
  - 线性代数
  - 数学
---
math: true

![](https://www.helloimg.com/i/2024/11/12/673343d45928a.jpg)

## 投影矩阵

对于任何矩阵$\left( 即任意的 \boldsymbol{M}\times N 矩阵\right)$，都可以通过Gram－Schmidt方法进行正交化，从而分解成一个正交矩阵 $A\times \boldsymbol{A}^{T} = 0$和一个上三角矩阵的的乘积，也就是我们所说的QR分解，其中Q为正交矩阵，R为上三角矩阵。由此，我们可以得到投影矩阵$\boldsymbol{A}_P$的一种计算方法，即 $\boldsymbol{A_{P}} = Q\times Q^{T}$ 。

对于一阶的$R^1$，由于显然其是否经过正交化与得到的结果仅为相差常数倍的关系，得到的结果并没有本质上的不同，因此对于向量$a$来说，其余向量在$a$上的投影矩阵显然是$a\times a^T$。
而对于二阶及以上的矩阵平面或者空间来说，显然就没有这么简单了。因此需要经过正交化后再进行处理。

## 最小二乘

在实际条件中，我们往往无法得到一个最完美的解用来满足$\boldsymbol{A}x = b$，因此我们需要找到一个x使得得到的解尽可能接近所需的解，即$\hat{x}$满足$\boldsymbol{A}\times \hat{x} = b$,
此时，可以左右同时乘以$\boldsymbol{A}^T$使得其能够满足投影矩阵条件，此式可以通过偏导数加以证明，但是我不会（。

## 应用

由最小二乘和投影矩阵的结合，我们可以得到一个最小二乘中的$\hat{x}$的解法，即$\boldsymbol{A}^{T} A\hat{x} = \boldsymbol{A}^{T}b$，此式也可以写成 \boldsymbol{A}\hat{x} = Q^TQb 的形式，这也是我要记录的重点。
其中Q是Matrix $\boldsymbol{A}$经过Gram-Schmidt方法正交化得到的产物。

在经过这样的过程后求得的$\hat{x}$不仅可以是最小二乘法的解，也可以是$\min\lvert\lvert b-\boldsymbol{A}x\rvert\rvert$ 。

## 补充，SVD分解和最小二乘的关系

我们知道，任何矩阵都可以分解成$A=U\sigma V^{T} $
的形式，因此，矩阵的广义逆$ A^+ $ 可以表示为$V \Sigma^{+} U^{T}$的形式，其中V和U前面已经说明，而$\Sigma^+$ 是$\Sigma$ 的一个广义逆，计算方法是将 $\Sigma$上的对角线元素取倒数后进行转置。由此可以算出矩阵的广义逆。

接着往下讲，$ x=A^{+} b+y-A^{+} Ay $ ，这是通过SVD变换和广义逆进行求解的方法。这个方法与上述所讲的$\boldsymbol{QR}$ 分解的本质是一样，但是使用范围更广。
