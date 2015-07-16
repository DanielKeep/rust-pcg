use PcgGenerator;
use bounds::NextBoundedResult;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntoIter<Rng>
where Rng: PcgGenerator {
    rng: Rng,
}

impl<Rng> IntoIter<Rng>
where Rng: PcgGenerator {
    pub fn new(rng: Rng) -> Self {
        IntoIter {
            rng: rng,
        }
    }
}

impl<Rng> Iterator for IntoIter<Rng>
where Rng: PcgGenerator {
    type Item = Rng::Result;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.next())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IntoIterBounded<Rng>
where
    Rng: PcgGenerator,
    Rng::Result: Clone + NextBoundedResult,
{
    rng: Rng,
    upper_bound: Rng::Result,
}

impl<Rng> IntoIterBounded<Rng>
where
    Rng: PcgGenerator,
    Rng::Result: Clone + NextBoundedResult,
{
    pub fn new(rng: Rng, upper_bound: Rng::Result) -> Self {
        IntoIterBounded {
            rng: rng,
            upper_bound: upper_bound,
        }
    }
}

impl<Rng> Iterator for IntoIterBounded<Rng>
where
    Rng: PcgGenerator,
    Rng::Result: Clone + NextBoundedResult,
{
    type Item = Rng::Result;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.next_bounded(self.upper_bound.clone()))
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct IntoIterMut<'a, Rng>
where Rng: 'a + PcgGenerator {
    rng: &'a mut Rng,
}

impl<'a, Rng> IntoIterMut<'a, Rng>
where Rng: 'a + PcgGenerator {
    pub fn new(rng: &'a mut Rng) -> Self {
        IntoIterMut {
            rng: rng,
        }
    }
}

impl<'a, Rng> Iterator for IntoIterMut<'a, Rng>
where Rng: 'a + PcgGenerator {
    type Item = Rng::Result;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.next())
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct IntoIterMutBounded<'a, Rng>
where
    Rng: 'a + PcgGenerator,
    Rng::Result: Clone + NextBoundedResult,
{
    rng: &'a mut Rng,
    upper_bound: Rng::Result,
}

impl<'a, Rng> IntoIterMutBounded<'a, Rng>
where
    Rng: 'a + PcgGenerator,
    Rng::Result: Clone + NextBoundedResult,
{
    pub fn new(rng: &'a mut Rng, upper_bound: Rng::Result) -> Self {
        IntoIterMutBounded {
            rng: rng,
            upper_bound: upper_bound,
        }
    }
}

impl<'a, Rng> Iterator for IntoIterMutBounded<'a, Rng>
where
    Rng: 'a + PcgGenerator,
    Rng::Result: Clone + NextBoundedResult,
{
    type Item = Rng::Result;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.rng.next_bounded(self.upper_bound.clone()))
    }
}
