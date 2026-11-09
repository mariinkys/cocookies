//! `?page=0&size=10&sortBy=title&sortDir=asc&search=pie` ⇄ [`PageQuery`], and the paged JSON
//! response `{content, page, size, totalElements, totalPages, last}`.

use serde::{Deserialize, Serialize};

use crate::{
    app_error::{AppError, AppResult},
    entities::{
        page::{DEFAULT_PAGE_SIZE, Page, PageQuery, SortDirection},
        recipe::RecipeSort,
        taxonomy::TaxonomySort,
    },
};

#[derive(Debug, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PageParams {
    pub page: Option<i64>,
    pub size: Option<i64>,
    pub sort_by: Option<String>,
    pub sort_dir: Option<String>,
    pub search: Option<String>,
}

/// A sort field addressable from the API by its public (camelCase) name.
pub trait ApiSortField: Sized + Default {
    fn from_api_name(name: &str) -> Option<Self>;
}

impl ApiSortField for RecipeSort {
    fn from_api_name(name: &str) -> Option<Self> {
        Some(match name {
            "title" => Self::Title,
            "createdAt" => Self::CreatedAt,
            "updatedAt" => Self::UpdatedAt,
            "prepTime" => Self::PrepTime,
            "cookTime" => Self::CookTime,
            "servings" => Self::Servings,
            _ => return None,
        })
    }
}

impl ApiSortField for TaxonomySort {
    fn from_api_name(name: &str) -> Option<Self> {
        Some(match name {
            "name" => Self::Name,
            "createdAt" => Self::CreatedAt,
            "updatedAt" => Self::UpdatedAt,
            _ => return None,
        })
    }
}

impl PageParams {
    pub fn into_query<S: ApiSortField>(self) -> AppResult<PageQuery<S>> {
        let page = self.page.unwrap_or(0);
        if page < 0 {
            return Err(AppError::Validation(
                "Page index must not be negative".into(),
            ));
        }
        let sort = match self
            .sort_by
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
        {
            None => S::default(),
            Some(name) => S::from_api_name(name)
                .ok_or_else(|| AppError::Validation(format!("Cannot sort by '{name}'")))?,
        };
        let direction = match self.sort_dir.as_deref() {
            Some(d) if d.eq_ignore_ascii_case("desc") => SortDirection::Desc,
            _ => SortDirection::Asc,
        };
        Ok(PageQuery::new(
            page,
            self.size.unwrap_or(DEFAULT_PAGE_SIZE),
            sort,
            direction,
            self.search,
        ))
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PageResponse<T> {
    pub content: Vec<T>,
    pub page: i64,
    pub size: i64,
    pub total_elements: i64,
    pub total_pages: i64,
    pub last: bool,
}

impl<T> From<Page<T>> for PageResponse<T> {
    fn from(p: Page<T>) -> Self {
        Self {
            content: p.content,
            page: p.page,
            size: p.size,
            total_elements: p.total_elements,
            total_pages: p.total_pages,
            last: p.last,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_and_whitelists() {
        let q: PageQuery<RecipeSort> = PageParams::default().into_query().unwrap();
        assert_eq!(
            (q.page, q.size, q.sort, q.direction),
            (0, 10, RecipeSort::Title, SortDirection::Asc)
        );

        let q: PageQuery<TaxonomySort> = PageParams {
            sort_by: Some("createdAt".into()),
            sort_dir: Some("DESC".into()),
            size: Some(1000),
            ..Default::default()
        }
        .into_query()
        .unwrap();
        assert_eq!(
            (q.sort, q.direction, q.size),
            (TaxonomySort::CreatedAt, SortDirection::Desc, 100)
        );

        let bad = PageParams {
            sort_by: Some("title; DROP TABLE recipes".into()),
            ..Default::default()
        };
        assert!(bad.into_query::<RecipeSort>().is_err());
        let neg = PageParams {
            page: Some(-1),
            ..Default::default()
        };
        assert!(neg.into_query::<RecipeSort>().is_err());
    }
}
