# OS实验1

## 去掉panic之后会发生什么？
rust_main函数是entry中_start函数调用的，执行结束之后会返回entry.asm。但是后面并没有任何代码，所以会继续执行后面的代码。
在各个函数中加入输出，根据输出结果来看，最终程序会不停地调用handle_interrupts函数。