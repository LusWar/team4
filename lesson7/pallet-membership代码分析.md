A-分析add_member的计算复杂度

add_member主要流程分几个步骤：
1. 在已排序members vector上做binary search找到插入新member的合适位置，这部分的时间复杂度为O(logN)
2. 在找到的位置插入新member，这部分时间复杂度最差情况为O(N)
3. change_members_sorted，这部分由于可以有不同的实现，因为时间复杂度未知
总体而言，add_member的时间复杂度至少为O(N)

B-分析pallet-membership是否适合以下场景下使用，提供原因：
* 存储预言机提供者
* 存储游戏链中每个公会的会员
* 存储PoA网络验证

pallet-membership提供了非常简单的存储结构——一个member向量和一个可有可无的主会员（prime member），还有时间复杂度相对较高的增删会员操作，使它只适合少量会员扁平结构的应用场景，所以它适合预言机提供者和PoA网络验证节点这两个场景，而不太适合游戏公会（某些游戏公会会员数以千计，并且有多层的组织结构）