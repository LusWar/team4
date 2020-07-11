# 作业

## 补完剩下的代码
`team4/lesson7/pallets/kitties/src/linked_item.rs`

## 修复单元测试
`team4/lesson7/pallets/kitties/src/lib.rs:305`

## 阅读 pallet-membership
* 分析 add_member 的计算复杂度

```
/// 总和: O(MP + N + logN)
/// 另外: 5次DB读取, 5+P次DB写入
#[weight = 50_000_000]
pub fn add_member(origin, who: T::AccountId) {

    // 内存: O(1) 
    T::AddOrigin::ensure_origin(origin)?;

    // 磁盘: 一次DB读取. 编码解码: O(n)
    let mut members = <Members<T, I>>::get();

    // 内存: O(log(n)) 
    let location = members.binary_search(&who).err().ok_or(Error::<T, I>::AlreadyMember)?;

    // 内存: O(n) 最坏情况全体后移
    members.insert(location, who.clone());

    // 磁盘: 一次DB写入. 编码解码: O(n)
    <Members<T, I>>::put(&members);

    // T::MembershipChanged 假设配置的是 collective-pallet
    // 需要查看:
    // frame/collective/src/lib.rs:848 `fn change_members_sorted(...){...}`
	// - `O(MP + N)`
	//   - where `M` old-members-count (governance-bounded)
	//   - where `N` new-members-count (governance-bounded)
	//   - where `P` proposals-count
    // 磁盘: 一次DB读取. Self::proposals() 编码解码: O(P)
    // 磁盘: <Voting<T, I>>::mutate 写入P次, 涉及M个成员, 编码解码:O(M)
    // 磁盘: Members::<T, I>::put(new)写入 编码解码: O(n)
    // 磁盘: Prime::<T, I>::kill()写入 编码解码: O(1)
    T::MembershipChanged::change_members_sorted(&[who], &[], &members[..]);
    
    // 需要到 substrate/frame/system/src/lib.rs 里面分析函数:
    // pub fn deposit_event_indexed(topics: &[T::Hash], event: T::Event) {...}
    // 这里topics是空, 我们没有topics.
    // 磁盘: Self::block_number(); 一次磁盘读取
    // 磁盘: ExecutionPhase::get(); 一次磁盘读取
    // 磁盘: EventCount::get(); 一次磁盘读取
    // 磁盘: EventCount::put(new_event_count); 一次磁盘写入
    // 磁盘: Events::<T>::append(&event); 一次磁盘写入
    Self::deposit_event(RawEvent::MemberAdded);
}
```

* 分析 pallet-membership 是否适合以下场景下使用,提供原因
  * 储存预言机提供者
    `人数不多可以保存, 并且修改不频繁`
  * 储存游戏链中每个工会的成员
    `人数太多不能保存, 并且修改频繁.`
  * 储存 PoA 网络验证人
    `人数不多可以保存, PoA验证人相对固定, 并且修改不频繁.`
