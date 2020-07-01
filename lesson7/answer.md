# Add_MemberShip复杂度计算

```
pub fn add_member(origin, who: T::AccountId) {
    # 复杂度O(1)
    T::AddOrigin::ensure_origin(origin)?;

    # 复杂度 O(n)
    let mut members = <Members<T, I>>::get();
    # 复杂度 O(logN)
    let location = members.binary_search(&who).err().ok_or(Error::<T, I>::AlreadyMember)?;
    members.insert(location, who.clone());
    # 复杂度 O(n)
    <Members<T, I>>::put(&members);

    # 复杂度O(n)
    T::MembershipChanged::change_members_sorted(&[who], &[], &members[..]);

    Self::deposit_event(RawEvent::MemberAdded);
}
```

# 分析 pallet-membership是否符合以下场景使用，提供原因

储存预言机提供者  
预言机提供者数量不会太多,该场景适合.  

储存游戏链中每个工会的成员 
游戏中的所有成员数量巨大,该场景不适合.  

储存POA网络验证 
POA验证节点数量也较少,该场景适合