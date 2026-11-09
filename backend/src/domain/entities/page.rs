//! Paging, sorting and free-text search for listings.

pub const DEFAULT_PAGE_SIZE: i64 = 10;
pub const MAX_PAGE_SIZE: i64 = 100;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortDirection {
    #[default]
    Asc,
    Desc,
}

/// A request for one page of a listing sorted by `S`.
#[derive(Debug, Clone)]
pub struct PageQuery<S> {
    /// 0-based page index.
    pub page: i64,
    pub size: i64,
    pub sort: S,
    pub direction: SortDirection,
    /// Free-text filter (trimmed, never empty).
    pub search: Option<String>,
}

impl<S: Default> Default for PageQuery<S> {
    fn default() -> Self {
        Self {
            page: 0,
            size: DEFAULT_PAGE_SIZE,
            sort: S::default(),
            direction: SortDirection::Asc,
            search: None,
        }
    }
}

impl<S> PageQuery<S> {
    /// Builds a query, clamping the page size and normalising the search term.
    pub fn new(
        page: i64,
        size: i64,
        sort: S,
        direction: SortDirection,
        search: Option<String>,
    ) -> Self {
        Self {
            page: page.max(0),
            size: size.clamp(1, MAX_PAGE_SIZE),
            sort,
            direction,
            search: search
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty()),
        }
    }

    pub fn offset(&self) -> i64 {
        self.page * self.size
    }
}

#[derive(Debug, Clone)]
pub struct Page<T> {
    pub content: Vec<T>,
    pub page: i64,
    pub size: i64,
    pub total_elements: i64,
    pub total_pages: i64,
    pub last: bool,
}

impl<T> Page<T> {
    pub fn new<S>(content: Vec<T>, query: &PageQuery<S>, total_elements: i64) -> Self {
        let total_pages = (total_elements + query.size - 1) / query.size;
        Self {
            content,
            page: query.page,
            size: query.size,
            total_elements,
            total_pages,
            last: query.page + 1 >= total_pages,
        }
    }

    pub fn map<U>(self, f: impl FnMut(T) -> U) -> Page<U> {
        Page {
            content: self.content.into_iter().map(f).collect(),
            page: self.page,
            size: self.size,
            total_elements: self.total_elements,
            total_pages: self.total_pages,
            last: self.last,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_math() {
        let q = PageQuery::new(1, 5, (), SortDirection::Asc, Some("  ".into()));
        assert_eq!(q.search, None);
        assert_eq!(q.offset(), 5);
        let p = Page::new(vec![1, 2], &q, 7);
        assert_eq!((p.total_pages, p.last), (2, true));
        let p = Page::<i32>::new(vec![], &q, 0);
        assert_eq!((p.total_pages, p.last), (0, true));
        assert_eq!(
            PageQuery::new(-3, 1000, (), SortDirection::Desc, None).size,
            MAX_PAGE_SIZE
        );
    }
}
