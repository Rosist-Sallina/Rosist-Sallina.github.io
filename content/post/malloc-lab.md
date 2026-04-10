---
title: "Malloc_Lab"
date: 2025-01-06T00:00:00+08:00
categories:
  - 学习笔记
tags:
  - C语言
  - 内存管理
  - 系统编程
---

# Malloc Lab

- 注:本文是笔者稍稍改动后的实验报告，有些部分由于认为助教可以理解故不做过多阐述请见谅。

- 又注：本实验相较于原版实验，对齐值部分改为了16字节，因此部分实现不同，请读者不要直接套用。由于直接套用导致的一切责任由读者承担。

## 1.基本数据组织形式

### 1.1 简介

本实现采用了分离适配的内存分配策略。事实上，我们将数据划分到不同的block中，每个Block都进行了16字节对齐以方便内存管理。其中，每个block都必定包含一个8字节的头部，一个8字节的尾部。在这种情况下，我们可以很方便的通过`bp - WSIZE`和`bp + DSIZE`来访问block的头部和尾部。
我们通过20个大小类的分离空闲链表(`segregated_lists[CLASS_SIZE]`)来管理空闲block。
每个链表都会管理特定大小范围的空闲block。
事实上，我们还对每一个链表维护了其大小顺序的排序，能够在`best_fit`的策略下增加查找效率。

### 内存块结构

- 已分配块:每一个已分配块都包含一个8字节的头部，一个8字节的尾部，以及一个有效载荷。其中头部和尾部都包含了一个分配位以表示当前块是否被分配。我们使用位运算 `& 0x1` 来获取分配状态。而使用位运算 `& ~0xF` 来获取块大小（清除低4位的标志位）。

- 空闲块:每一个空闲块都包含一个8字节头部，一个8字节的前驱指针，一个8字节的后驱指针，以及一个8字节为部，中间的位置空出。前驱指针和后驱指针是用来维护一个双向链表，能够更好的进行空闲块的查询和合并。。具体实现见下图。![](https://www.helloimg.com/i/2024/12/28/676ef42f60d5d.jpg)

## 2.相关操作和函数

### 分离适配

我们使用使用多个大小类的空闲链表(segregated lists)来管理空闲块，使用`get_class()`函数来为空闲块分配合适的链表。我们对于大小类的划分采用预定义的阈值：24, 32, 48, 64, 96, 128…等，并且对于较小的块我们采取了LIFO策略，而对于较大的块我们采取简单的大小排序的方式进行分配。

| 123456789101112 | static int get_class(size_t size) {    static const size_t class_sizes[] = {        24, 32, 48, 64, 96, 128, 192, 256, 384, 512,         768, 1024, 1536, 2048, 3072, 4096, 6144, 8192,         12288, 16384, 24576, 32768, 49152, 65536, 98304    };        for (int i = 0; i         if (size return i;    }    return CLASS_SIZE - 1;} |
| --- | --- |

### 搜索策略

我们采用了`beat_fit()`策略来搜索合适的空闲块。在这种情况下，我们的效率会稍微下降，但是内存的利用率会大幅上升。具体策略为:

- 首先在对应大小类中寻找

- 如果找不到合适的，向更大的类中搜索

- 对于完全匹配的块直接返回，否则选择差值最小的块

| 123456789101112131415161718192021222324252627282930 | static void *find_fit(size_t asize) {    void *best_fit = NULL;    size_t min_diff = (size_t)-1;    int cls = get_class(asize);        void *bp = segregated_lists[cls];    while (bp != NULL) {        size_t bsize = GET_SIZE(HDRP(bp));        if (bsize == asize) {  // ????            return bp;        }        if (bsize > asize) {            size_t diff = bsize - asize;            if (diff                 min_diff = diff;                best_fit = bp;                if (diff break;            }        }        bp = GET_SUCC(bp);    }        if (best_fit) return best_fit;        for (cls++; cls         bp = segregated_lists[cls];        if (bp != NULL) return bp;    }    return NULL;} |
| --- | --- |

### 分割策略

因为考虑到出现内存覆盖的问题，我们采用了一种较为简单的分割策略实现方式。即只有当剩余空间大于等于32字节时才进行分割，而分割出的空闲块会插入到对应的大小类中。

| 1234567891011121314151617 | static void place(void *bp, size_t asize) {    size_t bsize = GET_SIZE(HDRP(bp));    size_t remainder = bsize - asize;    delete_node(bp);    if (remainder >= 32) {        PUT(HDRP(bp), PACK(asize, 1));        PUT(FTRP(bp), PACK(asize, 1));        void *next_bp = NEXT_BLKP(bp);        PUT(HDRP(next_bp), PACK(remainder, 0));        PUT(FTRP(next_bp), PACK(remainder, 0));        insert_node(next_bp, remainder);    } else {        PUT(HDRP(bp), PACK(bsize, 1));        PUT(FTRP(bp), PACK(bsize, 1));    }} |
| --- | --- |

### 合并策略

我们采取了立即合并的策略。在对一个块进行操作后，我们会立即检查前后块的状态，合并相邻的空闲块，而合并后的块会直接插入到对应的大小类中。

| 123456789101112131415161718192021222324252627 | static void *coalesce(void *bp) {    size_t prev_alloc = GET_ALLOC(FTRP(PREV_BLKP(bp))) || PREV_BLKP(bp) == bp;    size_t next_alloc = GET_ALLOC(HDRP(NEXT_BLKP(bp)));    size_t size = GET_SIZE(HDRP(bp));    if (prev_alloc && next_alloc) {        insert_node(bp, size);        return bp;    }        if (!next_alloc) {        size += GET_SIZE(HDRP(NEXT_BLKP(bp)));        delete_node(NEXT_BLKP(bp));    }        if (!prev_alloc) {        size += GET_SIZE(HDRP(PREV_BLKP(bp)));        delete_node(PREV_BLKP(bp));        bp = PREV_BLKP(bp);    }        PUT(HDRP(bp), PACK(size, 0));    PUT(FTRP(bp), PACK(size, 0));    insert_node(bp, size);        return bp;} |
| --- | --- |

### realloc优化

如果新的大小等于当前块的大小，直接复用。如果不行，那么我们会尝试与相邻的空闲块合并以避免复制占用时间和效率。如果还是不被允许，那么我们才会考虑使用复制的方式腾出空间。

| 123456789101112131415161718192021222324252627282930313233343536 | void *mm_realloc(void *ptr, size_t size) {    if (ptr == NULL)        return mm_malloc(size);    if (size == 0) {        mm_free(ptr);        return NULL;    }    size_t oldsize = GET_SIZE(HDRP(ptr));    size_t newsize = ALIGN(size + DSIZE);        if (newsize         return ptr;    }    size_t next_alloc = GET_ALLOC(HDRP(NEXT_BLKP(ptr)));    size_t combined_size = oldsize;        if (!next_alloc) {        combined_size += GET_SIZE(HDRP(NEXT_BLKP(ptr)));    }        if (combined_size >= newsize) {        delete_node(NEXT_BLKP(ptr));        PUT(HDRP(ptr), PACK(combined_size, 1));        PUT(FTRP(ptr), PACK(combined_size, 1));        return ptr;    }        void *newptr = mm_malloc(size);    if (newptr == NULL)        return NULL;    memcpy(newptr, ptr, oldsize - DSIZE);    mm_free(ptr);    return newptr;} |
| --- | --- |

## 3.性能分析

我们针对较小的块$ \leq 256字节 $进行了优化，使用LIFO策略减少了查找和分配的时间。且，我们使用分离适配减少了内部碎片，使用立即合并策略有效减少了外部碎片。且我们针对realloc的扩展优化，解约了大量复制粘贴的时间。

## 4.可以优化的地方

- 在外部碎片的处理问题上，我们事实上可以采用延迟合并的方式，来提高某些trace情况下的性能。

- 我们可以添加块的边界标记，以更好的完成块之间的合并。

- 我们还可以使用更复杂的分离适配策略，来提高内存的利用率。（如：Buddy System）

## 5.完成实验的感想

本次实验不愧是CSAPP六次实验中难度最大的一个，也是代码实现最复杂的一个，也是优化最痛苦的一个，期间遇到了各种Segmentation Fault，以及各种内存覆盖之类杂七杂八的问题，De这种bug确实很麻烦。
事实上，个人觉得本次实验我还没有做到完美的程度，在代码的某些地方仍然存在不少优化空间，包括但不限于优化逻辑，以及喜闻乐见的调参。但是，完成本次实验仍然能让我很好的理解了malloc的原理以及实现，以及各种对于碎片内存的分配和处理方式，我相信，这也对我理解内存方面的各种知识有了很大的帮助。

## 6.参考

本次实验中，我参考了以下资料：

- CSAPP上对于该实验的相关实现

- 描述malloc lab的相关公开资料
[https://arthals.ink/blog/malloc-lab](https://arthals.ink/blog/malloc-lab)

- [https://zhuanlan.zhihu.com/p/640267967](https://zhuanlan.zhihu.com/p/640267967)等

## 7.附上实验结果

| 12345678910111213141516 | Results for mm malloc:trace            name     valid  util     ops      secs   Kops 1     amptjp-bal.rep       yes   99%    5694  0.000322  17672 2       cccp-bal.rep       yes   99%    5848  0.000308  18987 3    cp-decl-bal.rep       yes   99%    6648  0.000367  18134 4       expr-bal.rep       yes   99%    5380  0.000309  17439 5 coalescing-bal.rep       yes   99%   14400  0.000330  43663 6     random-bal.rep       yes   96%    4800  0.000532   9024 7    random2-bal.rep       yes   94%    4800  0.000511   9395 8     binary-bal.rep       yes   53%   12000  0.000701  17114 9    binary2-bal.rep       yes   47%   24000  0.000996  2410410    realloc-bal.rep       yes   78%   14401  0.000299  4818011   realloc2-bal.rep       yes   73%   14401  0.000190  75715Total                             85%  112372  0.004864  23103Score = (51 (util) + 40 (thru)) * 11/11 (testcase) = 56/100 |
| --- | --- |

- 魔改版满分60

好像在binary里面的效果还是很差哈哈，不知哪位大佬能否指点一下针对binary应该怎么再进行优化。

## 8.一些奇技淫巧

参考:[https://zhuanlan.zhihu.com/p/459355154](https://zhuanlan.zhihu.com/p/459355154)

| 12 | if(extend(2*DSIZE/WSIZE) == NULL)    return -1; |
| --- | --- |

在`mm_init`即初始化中分配的初始空间大小改为`2*DSIZE`能够提高初始块利用率。
经本人验证确实涨了2分xs
