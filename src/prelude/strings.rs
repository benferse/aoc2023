pub struct TrimmedLines<'a>(std::str::Lines<'a>);

impl<'a> Iterator for TrimmedLines<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(str::trim)
    }
}

pub struct MapLines<'a, F>(std::iter::Map<std::str::Lines<'a>, F>);

impl<'a, F, B> Iterator for MapLines<'a, F> where F: FnMut(&str) -> B {
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

pub trait StrExt<'a> {
    fn trimmed_lines(&self) -> TrimmedLines;
    fn map_lines<F, B>(&self, f: F) -> MapLines<F> where F: FnMut(&str) -> B;
}

impl<'a, T: AsRef<str>> StrExt<'a> for T {
    fn trimmed_lines(&self) -> TrimmedLines {
        TrimmedLines(self.as_ref().lines())
    }

    fn map_lines<F, B>(&self, f: F) -> MapLines<F> where F: FnMut(&str) -> B {
        MapLines(self.as_ref().lines().map(f))
    }
}
