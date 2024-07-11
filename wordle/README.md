# 大作业：Wordle

2024 年夏季学期《程序设计训练》 Rust 课堂大作业（一）。

## 项目说明

### 程序结构

```c
src
|-- app                                //仅包含app调用的函数
|    |-- judge_yew.rs                  //判断函数
|    |-- resources.rs       
|-- data
|   |-- config.json                      //配置文件
|-- app.rs                               //Web GUI
|-- builtin_words.rs                     //内置词库
|-- lib.rs                               //GUI入口
|-- judge.rs                             //判断函数
|-- main.rs                              //normal入口
|-- tty_mode.rs                          //终端输入模式
```
Crate下的各种模块（除builtin_words）为CLI独有，app下的模块为GUI独有.

### 用法说明

#### CLI
直接进行cargo run将会进入CLI交互模式。在此模式中，可以通过制定一些参数自定义游戏体验。

|  参数                     | 子参数 | 作用 | 备注 |
| -------------------------- | ------ |----------- | ----- |
|'--random' /'-r' | | 开启随机模式，会根据day和seed作为伪参数决定答案 | 依赖于'--random' |
|'--difficult' /'-D' | | 开启困难模式 |  |
|'--day' /'-d' | day<int> | 指定日期 | day的范围不能太大，否则会超出预选库大小 |
|'--seed' /'-s' | seed<int> | 指定种子 | 依赖于'--random' |
|'--word' /'-w' | word<String> | 指定答案 | 与'--random'冲突 |
| '--acceptable-set' / '-a' | path <String> | 指定可接受的单词集 |  |
| '--final-set' / '-f' | path <String> | 指定随机单词选取的的单词集 |  |
| '--state' / '-S' | path <String> | 指定状态存储路径 |  |
| '--stats' / '-t' | | 每一局结束后展示统计信息 |  |

#### GUI
本项目的GUI部分是基于Yew框架的Web应用，通过编译到WebAssembly的方式应用。目前本项目被部署在[这里](https://rosist-sallina.github.io/wordle/)。
通过键盘输入，一行输入完成后点击屏幕上的Enter键即可提交答案。对于无法提交的情况，请检查单词的合法性。

左上角的设置面板提供了困难模式的切换设置和种子的参数设置。

左边的统计面板会输出当前胜利局数和总局数，以及使用频率最高的5个单词。

值得注意的是，本项目实现了原Wordle游戏中单词翻转的动画，使变化效果更加美观圆滑。
#### 提高要求实现
采用[Yew](https://yew.rs/)框架以及极少量JavaScript（主要是为了实现界面的监听效果）实现。
并使用[Trunk](https://trunkrs.dev/)将编译完成的WebAssembly打包后，部署到Github Pages上。
