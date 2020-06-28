# lesson6


## 补充完整作业
```
team4/lesson6/pallets/kitties/src/lib.rs
```


## 对比 assets 和 balances

* 对于资金剩余很少的账户, balances是会将它删除的. 
  要看RefCount是否等于0
  reserved+free是否小于ExistentialDeposit
  删除后会释放空间, 对于全节点来说还是占用空间, 对于
  非全节点, 可以同步一部分最近的历史然后开始出块. 
  平时可以逐渐purge一部分最老的历史.
* 以太坊最近是越来越难同步了. 灰尘账户太多.
* assets缺少灰尘账户的删除
* assets缺少重要的trait的实现, 
  Currency, OnKilledAccount, OnUnbalanced,
  LockableCurrency, ReservableCurrency,
  这些trait可以对接其他模块,
  比如: System, Indices, nick, treasury, staking等.
* assets缺少reserved和lock的功能. 
* assets缺少权重的设计.
* assets缺少Imbalance的设计, 不好维护总发行量. 和做其他的计算. 


## 波卡的balance默认配置是u128不是u256

* 简单计算: (1<<128)-1=340282366920938463463374607431768211455=3.4e+38
* 使用最后12位做精度: 340282366920938463_463_374_607.431_768_211_455
* 能表示的数远远大于发行量1000万 = 1e+19.
  即使10%的速度增发51年, 也只是1.29e+21.
  即使50%的速度增发51年, 也只是9.56e+27.
* 对于计算来说, 我们给足够的空间可以用于加减乘除, 9.56e+27乘以1<<32都不会溢出.
* 以太坊使用u256, 总发行大概9000万Ether, 精度18位. 其实浪费很多空间.

