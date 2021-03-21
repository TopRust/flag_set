# FlagSet

[![Crate](https://img.shields.io/crates/v/flag_set)](https://crates.io/crates/flag_set) [![doc.rs](https://img.shields.io/docsrs/flag_set/0.1.6)](https://docs.rs/flag_set/)<br>

`HashSet` only implements the normal set. `HashSet` can not represent its complementary set when the complementary set is an infinite set.

`FlagSet` implemented as a tuple of a `HashSet` and a  `bool` value. When the `bool` value is true, `FlagSet` represents the `HashSet` .  When the `bool` value is true, `FlagSet` represents the the complementary set of `HashSet` .

As with the [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) type, a `FlagSet` requires that the elements implement the [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) traits. In addition to operations of `FlagSet`, the elements also implement the  [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait.

`FlagSet` also defines five kinds of operations of sets based on  [`Binary Operations`](http://www.unicode.org/reports/tr18/#Resolving_Character_Ranges_with_Strings).


A ∪B -> A + B 

A ∩ B -> A & B

A - B -> A - B

A Xor B -> A ^ B

C<sub>u</sub>A -> !A

# Examples

```rust
use std::collections::HashSet;
use flag_set::FlagSet;

let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

let flag_ap = FlagSet(a.clone(), true);
let flag_an = FlagSet(a.clone(), false);
let flag_bp = FlagSet(b.clone(), true);
let flag_bn = FlagSet(b.clone(), false);

// 用new方法创建实例
// use new method create an instance
assert_eq!(flag_bn, FlagSet::new(vec![2, 3, 4], false));

// 测试并
// test union
assert_eq!(FlagSet::new(vec![1, 2, 3, 4], true), flag_ap.clone() + flag_bp.clone());
assert_eq!(FlagSet::new(vec![4], false), flag_ap.clone() + flag_bn.clone());
assert_eq!(FlagSet::new(vec![1], false), flag_an.clone() + flag_bp.clone());
assert_eq!(FlagSet::new(vec![2, 3], false), flag_an.clone() + flag_bn.clone());

// 测试交
// test intersection
assert_eq!(FlagSet::new(vec![2, 3], true), flag_ap.clone() & flag_bp.clone());
assert_eq!(FlagSet::new(vec![1], true), flag_ap.clone() & flag_bn.clone());
assert_eq!(FlagSet::new(vec![4], true), flag_an.clone() & flag_bp.clone());
assert_eq!(FlagSet::new(vec![1, 2, 3, 4], false), flag_an.clone() & flag_bn.clone());

// 测试减
// test substraction
assert_eq!(FlagSet::new(vec![1], true), flag_ap.clone() - flag_bp.clone());
assert_eq!(FlagSet::new(vec![2, 3], true), flag_ap.clone() - flag_bn.clone());
assert_eq!(FlagSet::new(vec![1, 2, 3, 4], false), flag_an.clone() - flag_bp.clone());
assert_eq!(FlagSet::new(vec![4], true), flag_an.clone() - flag_bn.clone());

// 测试否
// test not
assert_eq!(FlagSet(a.clone(), true), !flag_an.clone());

// 测试对称差
// test symmetric difference
assert_eq!(FlagSet::new(vec![1, 4], true), flag_ap.clone() ^ flag_bp.clone());
```

