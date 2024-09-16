pub use flagger_macros::flags;

#[flags]
#[derive(Debug)]
enum TestEnum {
    First = 1,
    Second = 2,
    Third = 4,
    //Fourth,

    FirstAndSecond = Self::First | Self::Second
}

#[cfg(test)]
mod tests {
    use super::*;
    use should::*;
    
    #[test]
    fn variants_exist() {
        Into::<u32>::into(TestEnum::None).should_be(0);
        Into::<u32>::into(TestEnum::First).should_be(1);
        Into::<u32>::into(TestEnum::Second).should_be(2);
        Into::<u32>::into(TestEnum::Third).should_be(4);
        //Into::<u32>::into(TestEnum::Fourth).should_be(8);
        Into::<u32>::into(TestEnum::FirstAndSecond).should_be(3);
    }

    #[test]
    fn can_test_for_flags() {
        TestEnum::First.intersects(TestEnum::First).should_be(true);
        TestEnum::First.intersects(TestEnum::Second).should_be(false);
        TestEnum::First.intersects(TestEnum::FirstAndSecond).should_be(true);
        TestEnum::First.contains(TestEnum::First).should_be(true);
        TestEnum::First.contains(TestEnum::Second).should_be(false);
        TestEnum::First.contains(TestEnum::FirstAndSecond).should_be(false);

        TestEnum::Second.intersects(TestEnum::First).should_be(false);
        TestEnum::Second.intersects(TestEnum::Second).should_be(true);
        TestEnum::Second.intersects(TestEnum::FirstAndSecond).should_be(true);
        TestEnum::Second.contains(TestEnum::First).should_be(false);
        TestEnum::Second.contains(TestEnum::Second).should_be(true);
        TestEnum::Second.contains(TestEnum::FirstAndSecond).should_be(false);

        TestEnum::FirstAndSecond.intersects(TestEnum::First).should_be(true);
        TestEnum::FirstAndSecond.intersects(TestEnum::Second).should_be(true);
        TestEnum::FirstAndSecond.intersects(TestEnum::FirstAndSecond).should_be(true);
        TestEnum::FirstAndSecond.contains(TestEnum::First).should_be(true);
        TestEnum::FirstAndSecond.contains(TestEnum::Second).should_be(true);
        TestEnum::FirstAndSecond.contains(TestEnum::FirstAndSecond).should_be(true);
    }

    #[test]
    fn works_with_derive_macros() {
        println!("{:?}", TestEnum::First);
    }
}