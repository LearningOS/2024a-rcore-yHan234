# Lab3 report

## 功能总结

实现 spawn：参考 `INITPROC` 的方式新建一个 `Arc<TaskControlBlock>`，参考 `fork` 更新 parent child 信息。

实现 Strike：为 `TaskControlBlockInner` 添加 strike 相关字段，为 `TaskControlBlock` 实现 `Ord` Trait，将 TaskManager 改为 BinaryHeap 实现小根堆。

## stride 算法深入

实际情况是轮到 p1 执行吗？为什么？不是，因为 stride 溢出了。

为什么优先级 >= 2 就可以使 `STRIDE_MAX – STRIDE_MIN <= BigStride / 2`？假设仅有两个进程，若它们 stride 不相等，则调度小的；若相等，则任意调度。在这两种情况，由于 `pass <= BigStride / 2`，必然不会造成 `STRIDE_MAX – STRIDE_MIN > BigStride / 2`。

```rust
use core::cmp::Ordering;

struct Stride(u64);
const BIG_STRIDE: usize = 255;
const HALF_BIG_STRIDE: usize = BIG_STRIDE / 2;

impl PartialOrd for Stride {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.0 == other.0 {
            Some(Ordering::Equal)
        } else if self.0 < other.0 {
            if other.0 - self.0 > half_big_stride {
                Some(Ordering::Greater) // self 溢出
            } else {
                Some(Ordering::Less)
            }
        } else {
            if self.0 - other.0 > half_big_stride {
                Some(Ordering::Less) // other 溢出
            } else {
                Some(Ordering::Greater)
            }
        }
    }
}

impl PartialEq for Stride {
    fn eq(&self, other: &Self) -> bool {
        false
    }
}
```

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。
