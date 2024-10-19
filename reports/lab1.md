# Lab1 report

## 功能总结

在 TaskControlBlock 新增两个字段：`task_start_time` 记录 Task 启动时间, `syscall_times` 数组记录系统调用次数。

TaskManager 新增两个方法：`current_syscall_count(syscall_id)` 为当前 Task 的对应 syscall_times 加一，`get_current_task_control_block` 获取当前 Task 的 TaskControlBlock 指针。

在 `syscall` 中调用 `current_syscall_count(syscall_id)` 更新系统调用次数。

实现 `sys_task_info`：调用 `get_current_task_control_block` 获取信息并写入 TaskInfo。

## 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。

CPU 发现非法行为，设置 `mepc` 和 `mcause`，跳转至 `mtvec`。

`mtvec` 保存上下文后跳转至 `trap_handler`，`trap_handler` 根据 `mcause` 发现用户非法行为，打印错误信息并切换执行其它程序。

## 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:

### L40：刚进入 __restore 时，a0 代表了什么值。请指出 __restore 的两种使用情景。

a0 代表了进入 trap 的用户进程被 `__alltraps` 保存的上下文（不会被 `__restore` 使用）。

一种使用情景是构建一个新的 Task 上下文时，会将 `ra` 设置为 `__restore`。

另一种使用情景是在 Trap 返回时会自动返回至 `__restore`，因为 trap.S 中 `__restore` 就在 `call trap_handler` 之后。

### L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。

```asm
ld t0, 32*8(sp)
ld t1, 33*8(sp)
ld t2, 2*8(sp)
csrw sstatus, t0
csrw sepc, t1
csrw sscratch, t2
```

- sstatus: 将 SPP 设为 0 以在 `sret` 时返回到 U 模式。
- sepc: `sret` 将跳转到的地址。
- sscratch: 用户栈的地址，之后会交换 sscratch 和 sp 以切换到用户栈。

### L50-L56：为何跳过了 x2 和 x4？

```asm
ld x1, 1*8(sp)
ld x3, 3*8(sp)
.set n, 5
.rept 27
   LOAD_GP %n
   .set n, n+1
.endr
```

x2 是 sp，它通过 sscratch 进行切换。

x4 是 tp，因为（实验文档里说）它不会被用到。

### L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？

```asm
csrrw sp, sscratch, sp
```

现在 sp 为用户栈指针，sscratch 为内核栈指针。

### __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？

`sret`。因为 sstatus 中的 SPP 被设为了 0。

### L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？

```
csrrw sp, sscratch, sp
```

现在 sp 为内核栈指针，sscratch 为用户栈指针。

### 从 U 态进入 S 态是哪一条指令发生的？

`ecall` 或非法行为。

## 荣誉准则

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：
2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：
3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。
4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。

## 本次实验的看法

较简单，工作量较少。

希望可以与“上下文切换”联系更加紧密。

可以在问答题中加入 `__switch` 相关题目。
