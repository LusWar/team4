# Homework for lesson 12

答案放在README文件里即可。

## 1. 为 template 模块的 do_something 添加 benchmark 用例（也可以是其它自选模块的可调用函数），并且将 benchmark 运行的结果转换为对应的权重定义；

Note: 上传benchmark运行结果的命令行截图和最终的可调用函数代码片段（包含权重设置）。

```
➜  lesson12 git:(lesson12) make benchmarks 
cd node;cargo build --release --features runtime-benchmarks
   Compiling node-template-runtime v2.0.0-rc2 (/data/test/team4/lesson12/runtime)
   Compiling pallet-template v2.0.0-rc2 (/data/test/team4/lesson12/pallets/template)
   Compiling node-template v2.0.0-rc2 (/data/test/team4/lesson12/node)
    Finished release [optimized] target(s) in 1m 47s
target/release/node-template benchmark \
        --chain=dev \
        --execution=wasm \
        --wasm-execution=compiled \
        --pallet=pallet-template \
        --extrinsic=do_something \
        --steps=20 \
        --repeat=51
Pallet: "pallet-template", Extrinsic: "do_something", Lowest values: [], Highest values: [], Steps: [20], Repeat: 51
Median Slopes Analysis
========

Model:
Time ~=    14.81
    + b        0
              µs

Min Squares Analysis
========

Data points distribution:
    b   mean µs  sigma µs       %
    1     14.96     0.128    0.8%
   50      14.8     0.052    0.3%
   99     14.77     0.055    0.3%
  148     14.73     0.039    0.2%
  197     14.87     0.146    0.9%
  246     14.75     0.042    0.2%
  295     14.76     0.032    0.2%
  344      14.8     0.067    0.4%
  393     14.75     0.047    0.3%
  442     14.72     0.058    0.3%
  491     14.69     0.052    0.3%
  540     14.74      0.05    0.3%
  589     14.67     0.053    0.3%
  638     14.69     0.064    0.4%
  687     14.71     0.044    0.2%
  736      14.7     0.071    0.4%
  785     14.73      0.06    0.4%
  834     14.67     0.069    0.4%
  883      14.7     0.058    0.3%
  932     14.68     0.053    0.3%
  981     14.71     0.044    0.2%

Quality and confidence:
param     error
b             0

Model:
Time ~=    14.83
    + b        0
              µs

```

```
/// # <weight>
/// - Base Weight: 14.83 µs
/// - DB Weight: 1 Write
/// # </weight>
#[weight = T::DbWeight::get().writes(1) + 15*1_000_000]
pub fn do_something(origin, something: u32) -> dispatch::DispatchResult {
```


## 2. 选择 node-template 或者其它节点程序，生成 Chain Spec 文件（两种格式都需要）；

Note: 上传 Chain Spec 文件即可

## 3.（附加题）根据 Chain Spec，部署公开测试网络。

Note: 上传 telemetry.polkadot.io 上你的网络节点的截图，或者apps上staking页面截图。



