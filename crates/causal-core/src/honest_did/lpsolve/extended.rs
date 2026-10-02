//! IEEE binary128 accumulation, matching long double in the arm64 Linux oracle.
use rustc_apfloat::{
    ieee::{Double, Quad},
    Float, FloatConvert,
};
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct Extended(Quad);
impl Extended {
    pub const ZERO: Self = Self(Quad::ZERO);
    pub fn new<T: Into<f64>>(value: T) -> Self {
        let value = Double::from_bits(value.into().to_bits() as u128);
        Self(value.convert(&mut false).value)
    }
    pub fn to_f64(self) -> Option<f64> {
        let result: Double = self.0.convert(&mut false).value;
        Some(f64::from_bits(result.to_bits() as u64))
    }
}
macro_rules! operation {
    ($trait:ident,$method:ident,$assign:ident,$assign_method:ident,$op:tt)=>{
        impl core::ops::$trait for Extended {
            type Output=Self;
            fn $method(self,rhs:Self)->Self {Self((self.0 $op rhs.0).value)}
        }
        impl core::ops::$assign for Extended {
            fn $assign_method(&mut self,rhs:Self){*self=*self $op rhs;}
        }
    }
}
operation!(Add,add,AddAssign,add_assign,+);
operation!(Sub,sub,SubAssign,sub_assign,-);
operation!(Mul,mul,MulAssign,mul_assign,*);
operation!(Div,div,DivAssign,div_assign,/);
impl core::ops::Neg for Extended {
    type Output = Self;
    fn neg(self) -> Self {
        Self(-self.0)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accumulation_does_not_collapse_to_binary64() {
        let x = Extended::new(9007199254740992.0);
        assert_eq!((x + Extended::new(1.0) - x).to_f64(), Some(1.0));
        assert_eq!(
            (Extended::new(1.0) / Extended::new(3.0)).to_f64(),
            Some(1.0 / 3.0)
        );
    }
}
