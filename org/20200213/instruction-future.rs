pub trait InstAlloc<'s, 'c> {
    fn alloc_str(&'c self, text: &'s str) -> &'c str;
}

impl<'s, 'c> InstAlloc<'s, 'c> for Bump {
    #[inline(always)]
    fn alloc_str(&'c self, text: &'s str) -> &'c str {
        self.alloc_str(text)
    }
}

pub struct DirectInstAlloc;

impl<'s> InstAlloc<'s, 's> for DirectInstAlloc {
    #[inline(always)]
    fn alloc_str(&'s self, text: &'s str) -> &'s str {
        text
    }
}
