#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

//! `HashSet` only implements the normal set. `HashSet` can not represent its complementary set when the complementary set is an infinite set.
//! 
//! `FlagSet` implemented as a tuple of a `HashSet` and a  `bool` value. When the `bool` value is true, `FlagSet` represents the `HashSet`.  When the `bool` value is true, `FlagSet` represents the the complementary set of `HashSet`.
//! 
//! As with the [`HashSet`](https://doc.rust-lang.org/std/collections/struct.HashSet.html) type, a `FlagSet` requires that the elements implement the [`Eq`](https://doc.rust-lang.org/std/cmp/trait.Eq.html) and [`Hash`](https://doc.rust-lang.org/std/hash/trait.Hash.html) traits. In addition to operations of `FlagSet`, the elements also implement the  [`Clone`](https://doc.rust-lang.org/std/clone/trait.Clone.html) trait.
//! 
//! `FlagSet` also defines five kinds of operations of sets based on  [`Binary Operations`](http://www.unicode.org/reports/tr18/#Resolving_Character_Ranges_with_Strings).
//! 
//! 
//! A ∪B -> A + B 
//! 
//! A ∩ B -> A & B
//! 
//! A - B -> A - B
//! 
//! A Xor B -> A ^ B
//! 
//! C<sub>u</sub>A -> !A
//! 
//! # Examples
//! 
//! ```rust
//! use std::collections::HashSet;
//! use flag_set::FlagSet;
//! 
//! let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
//! let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();
//! 
//! let flag_ap = FlagSet(a.clone(), true);
//! let flag_an = FlagSet(a.clone(), false);
//! let flag_bp = FlagSet(b.clone(), true);
//! let flag_bn = FlagSet(b.clone(), false);
//! 
//! // 用new方法创建实例
//! // use new method create an instance
//! assert_eq!(flag_bn, FlagSet::new(vec![2, 3, 4], false));
//! 
//! // 测试并
//! // test union
//! assert_eq!(FlagSet::new(vec![1, 2, 3, 4], true), flag_ap.clone() + flag_bp.clone());
//! assert_eq!(FlagSet::new(vec![4], false), flag_ap.clone() + flag_bn.clone());
//! assert_eq!(FlagSet::new(vec![1], false), flag_an.clone() + flag_bp.clone());
//! assert_eq!(FlagSet::new(vec![2, 3], false), flag_an.clone() + flag_bn.clone());
//! 
//! // 测试交
//! // test intersection
//! assert_eq!(FlagSet::new(vec![2, 3], true), flag_ap.clone() & flag_bp.clone());
//! assert_eq!(FlagSet::new(vec![1], true), flag_ap.clone() & flag_bn.clone());
//! assert_eq!(FlagSet::new(vec![4], true), flag_an.clone() & flag_bp.clone());
//! assert_eq!(FlagSet::new(vec![1, 2, 3, 4], false), flag_an.clone() & flag_bn.clone());
//! 
//! // 测试减
//! // test substraction
//! assert_eq!(FlagSet::new(vec![1], true), flag_ap.clone() - flag_bp.clone());
//! assert_eq!(FlagSet::new(vec![2, 3], true), flag_ap.clone() - flag_bn.clone());
//! assert_eq!(FlagSet::new(vec![1, 2, 3, 4], false), flag_an.clone() - flag_bp.clone());
//! assert_eq!(FlagSet::new(vec![4], true), flag_an.clone() - flag_bn.clone());
//! 
//! // 测试否
//! // test not
//! assert_eq!(FlagSet(a.clone(), true), !flag_an.clone());
//! 
//! // 测试对称差
//! // test symmetric difference
//! assert_eq!(FlagSet::new(vec![1, 4], true), flag_ap.clone() ^ flag_bp.clone());
//! ```
//! 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {

        use std::collections::HashSet;
        use super::FlagSet;
        
        let a: HashSet<_> = vec![1, 2, 3].into_iter().collect();
        let b: HashSet<_> = vec![2, 3, 4].into_iter().collect();

        let flag_ap = FlagSet(a.clone(), true);
        let flag_an = FlagSet(a.clone(), false);
        let flag_bp = FlagSet(b.clone(), true);
        let flag_bn = FlagSet(b.clone(), false);
        
        // 用new方法创建实例
        // use new method create an instance
        assert_eq!(flag_bn, FlagSet::new(vec![2, 3, 4], false));

        // 测试contains方法
        // test contains
        assert!(flag_an.contains(4));

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
    }
}

use std::collections::HashSet;
use std::hash::Hash;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct FlagSet<T: Eq + Hash>(HashSet<T>, bool);

// impl Default
impl<T: Eq + Hash> Default for FlagSet<T> {
    /// 空集。<br>
    /// Creating an empty set.
    fn default() -> Self {
        Self(HashSet::<T>::with_capacity(0), true)
    }

}

// 实现了contains方法
// impl contains method
impl<'a, T: Eq + Hash + Clone> FlagSet<T> {

    /// 用vector和bool新建实例。<br>
    /// Ccreating an instance with a vector and a boolean value.
    pub fn new(vector: Vec<T>, flag: bool) -> Self {
        Self(vector.into_iter().collect(), flag)
    }
    /// 判断集合是否包含值。<br>
    /// Identifying whether set contains value
    pub fn contains(&self, value: T) -> bool {
        !(self.0.contains(&value) ^ self.1)
    }
}

use std::ops::{Add, BitAnd, Not, Sub, BitXor};

// 并
// union
impl<T: Eq + Hash + Clone> Add for FlagSet<T> {
    type Output = Self;
    /// A + B<br>
    /// A和B会销毁，所以用希望保留A和B的话，请使用A.clone() + B.clone()。<br>
    /// A and B will be consumed. If you want to reserve A and B, using A.clone() + B.clone() instead of A + B.
    fn add(self, other: Self) -> Self::Output {
        match (self, other) {

            (FlagSet(A, true), FlagSet(B, true)) => Self(&A | &B, true),
            (FlagSet(A, true), FlagSet(B, false)) => Self(&B - &A, false),
            (FlagSet(A, false), FlagSet(B, true)) => Self(&A - &B, false),
            (FlagSet(A, false), FlagSet(B, false)) => Self(&A & &B, false),
        }
    }
}

// 交
// intersection
impl<T: Eq + Hash + Clone> BitAnd for FlagSet<T> {
    type Output = Self;
    /// A & B<br>
    /// A和B会销毁，所以用希望保留A和B的话，请使用A.clone() & B.clone()。<br>
    /// A and B will be consumed. If you want to reserve A and B, using A.clone() & B.clone() instead of A & B.
    fn bitand(self, other: Self) -> Self::Output {
        match (self, other) {

            (FlagSet(A, true), FlagSet(B, true)) => Self(&A & &B, true),
            (FlagSet(A, true), FlagSet(B, false)) => Self(&A - &B, true),
            (FlagSet(A, false), FlagSet(B, true)) => Self(&B - &A, true),
            (FlagSet(A, false), FlagSet(B, false)) => Self(&A | &B, false),
        }
    }
}


// 差
// subtraction
impl<T: Eq + Hash + Clone> Sub for FlagSet<T> {
    type Output = Self;
    /// A - B<br>
    /// A和B会销毁，所以用希望保留A和B的话，请使用A.clone() - B.clone()。<br>
    /// A and B will be consumed. If you want to reserve A and B, using A.clone() - B.clone() instead of A - B.
    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {

            (FlagSet(A, true), FlagSet(B, true)) => Self(&A - &B, true),
            (FlagSet(A, true), FlagSet(B, false)) => Self(&A & &B, true),
            (FlagSet(A, false), FlagSet(B, true)) => Self(&A | &B, false),
            (FlagSet(A, false), FlagSet(B, false)) => Self(&B - &A, true),
        }
    }
}

// 对称差
// symmetric difference
impl<T: Eq + Hash + Clone> BitXor for FlagSet<T> {
    type Output = Self;
    /// A ^ B<br>
        /// A和B会销毁，所以用希望保留A和B的话，请使用A.clone() ^ B.clone()。<br>
    /// A and B will be consumed. If you want to reserve A and B, using A.clone() ^ B.clone() instead of A ^ B.
    fn bitxor(self, other: Self) -> Self::Output {
        (self.clone() + other.clone()) - (self & other)
    }
}

// 否
// not
impl<T: Eq + Hash + Clone> Not for FlagSet<T> {
    type Output = Self;
    /// !A<br>
    /// 不同于二（次）元操作，A不会销毁，只转换了A的布尔值。<br>
    /// ! operation differs from the above binary operations. A only transforms its bool value. A is not destroyed.
    fn not(mut self) -> Self::Output {
        self.1 = !self.1;
        self
    }
}
