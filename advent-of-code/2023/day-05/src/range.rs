#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct SeedRange {
    pub(crate) start: usize,
    pub(crate) length: usize,
}

impl SeedRange {
    pub(crate) fn new(start: usize, length: usize) -> Self {
        Self { start, length }
    }

    pub(crate) fn split_with_shift(
        self,
        other: &Self,
        shift: i64,
    ) -> (Vec<SeedRange>, Vec<SeedRange>) {
        // <----->
        //           <------>
        if self.start + self.length < other.start || other.start + other.length < self.start {
            return (vec![self], vec![]);
        }

        let self_starts_before_other = self.start < other.start;
        let self_ends_before_other = self.start + self.length <= other.start + other.length;

        match (self_starts_before_other, self_ends_before_other) {
            // <----->
            //      <------>
            (true, true) => (
                vec![Self::new(self.start, other.start - self.start)],
                vec![Self::new(
                    (other.start as i64 + shift) as usize,
                    self.start + self.length - other.start,
                )],
            ),
            // <---------->
            //     <--->
            (true, false) => (
                vec![
                    Self::new(self.start, other.start - self.start),
                    Self::new(
                        (other.start as i64 + other.length as i64) as usize,
                        self.start + self.length - other.start - other.length,
                    ),
                ]
                .into_iter()
                .filter(|range| range.length != 0)
                .collect(),
                vec![Self::new(
                    (other.start as i64 + shift) as usize,
                    other.length,
                )],
            ),
            //    <--->
            // <---------->
            (false, true) => (
                vec![],
                vec![Self::new((self.start as i64 + shift) as usize, self.length)],
            ),

            //           <----->
            //      <------>
            (false, false) => (
                vec![Self::new(
                    other.start + other.length,
                    self.start + self.length - other.start - other.length,
                )],
                vec![Self::new(
                    (self.start as i64 + shift) as usize,
                    other.start + other.length - self.start,
                )],
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SeedRange;

    #[test]
    fn test_split() {
        // <-->
        //       <---->
        let origin_range = SeedRange::new(10, 10);
        let split_range = SeedRange::new(0, 1);
        let res = origin_range.clone().split_with_shift(&split_range, 0);
        println!("{res:?}");
        // <----->
        //      <------>
        let origin_range = SeedRange::new(10, 10);
        let split_range = SeedRange::new(17, 3);
        let res = origin_range.clone().split_with_shift(&split_range, 1);
        println!("{res:?}");

        // <---------->
        //     <--->
        let origin_range = SeedRange::new(10, 10);
        let split_range = SeedRange::new(15, 3);
        let res = origin_range.clone().split_with_shift(&split_range, 1);
        println!("{res:?}");

        let origin_range = SeedRange::new(10, 10);
        let split_range = SeedRange::new(15, 5);
        let res = origin_range.clone().split_with_shift(&split_range, 1);
        println!("{res:?}");

        //    <--->
        // <---------->
        let origin_range = SeedRange::new(10, 10);
        let split_range = SeedRange::new(10, 10);
        let res = origin_range.clone().split_with_shift(&split_range, 1);
        println!("{res:?}");

        //           <----->
        //      <------>
        let origin_range = SeedRange::new(10, 10);
        let split_range = SeedRange::new(10, 5);
        let res = origin_range.clone().split_with_shift(&split_range, 1);
        println!("{res:?}");
    }
}
