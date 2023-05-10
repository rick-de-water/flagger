#[allow(non_upper_case_globals)]
pub trait Flags:
    std::ops::BitAnd<Self, Output = Self> +
    std::ops::BitAndAssign<Self> +
    std::ops::BitOr<Self, Output = Self> +
    std::ops::BitOrAssign<Self> +
    std::ops::BitXor<Self, Output = Self> +
    std::ops::BitXorAssign<Self> +
    std::ops::Not<Output = Self> +
    PartialEq + Eq +
    Copy + Sized {

    type Representation;
    
    const None: Self;
    const All: Self;
    
    fn has_any_flag(&self, flags: Self) -> bool {
        self.bitand(flags).ne(&Self::None)
    }

    fn has_all_flags(&self, flags: Self) -> bool {
        self.bitand(flags).eq(&flags)
    }
}

pub use flagger_macros::flags;

#[flags(dfsdfds)]
enum TestEnum {
    First = 1,
    Second = 2,
    Third = 4,

    FirstAndSecond = Self::First | Self::Second
}

#[cfg(test)]
mod tests {
    use super::*;
    use should::*;
    
    #[test]
    fn variants_exist() {
        Into::<u32>::into(TestEnum::First).should_be(1);
        Into::<u32>::into(TestEnum::Second).should_be(2);
        Into::<u32>::into(TestEnum::Third).should_be(4);
        Into::<u32>::into(TestEnum::FirstAndSecond).should_be(3);
    }
}