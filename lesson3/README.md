## 第三课作业  PoE 2

课程里会给出参考资料，大家一定要自己敲一遍**代码**！

注：

1. 提交源代码，运行`cargo test`的测试结果截图，前端UI的截图；
2. 测试应覆盖所有的业务逻辑，如不同的错误场景，以及一切正常的情况下，检查存储的数据是不是预期的那样。
3. 附加题不是必答的，但可以酌情加分。
4. 代码修改在本目录 substrate-node-template 和 substrate-front-end-template 的程序文件里。

第一题：编写存证模块的单元测试代码，包括：
  >![star](./images/lesson3-unit-tests.png)
* 创建存证的测试用例；
* 撤销存证的测试用例；
* 转移存证的测试用例；

第二题：编写存证模块的UI，包括

* 创建存证的UI
  >![star](./images/lesson3-create-claim-ui.png)
* 删除存证的UI
  >![star](./images/lesson3-revoke-claim-ui.png)
* 转移存证的UI
  >![star](./images/lesson3-transfer-claim-ui.png)

第三题（附加题）：实现购买存证的功能代码：

* 用户A为自己的某个存证记录设置价格；
* 用户B可以以一定的价格购买某个存证，当出价高于用户A设置的价格时，则以用户A设定的价格将费用从用户B转移到用户A，再将该存证进行转移。如果出价低于用户A的价格时，则不进行转移，返回错误。
> 创建存证
 >![star](./images/lesson3-q3-create-claim-ui.png)

> 创建存证事件
 >![star](./images/lesson3-q3-create-claim-event-ui.png)

> 查询存证初始价格
 >![star](./images/lesson3-q3-purchased-claim-price-query-ui.png)

> 为存证设置价格
 >![star](./images/lesson3-q3-attach-claim-price-ui.png)

> 查询设置后的存证价格
 >![star](./images/lesson3-q3-attached-claim-price-query-ui.png)

> 购买存证
 >![star](./images/lesson3-q3-purchased-claim-ui.png)

> 查询购买后的存证
 >![star](./images/lesson3-q3-purchased-claim-price-query-ui.png)
