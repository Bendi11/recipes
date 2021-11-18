pub mod data;
pub mod handler;
mod impls;
pub mod theme;
pub mod ui;
pub mod widgets;

use crate::recipes::db::RecipeId;

use self::data::screen::AppScreen;
use druid::Selector;
pub use ui::root_widget;

/// The visually-appealing golden ratio
pub const GOLDEN_RATIO: f64 = 1.61803;

/// Change the currently visisble screen command
pub const CHANGE_SCREEN: Selector<AppScreen> = Selector::new("recipier.change-screen");

/// Populate search results with data from the search query state
pub const POPULATE_RESULTS: Selector = Selector::new("recipier.populate-search-results");

/// View the specified recipe in the recipe view screen
pub const VIEW_RECIPE: Selector<RecipeId> = Selector::new("recipier.view-recipe");

/// Load more recipes into the recipes home screen
pub const LOAD_MORE_RECIPES: Selector = Selector::new("recipeier.load-more-recipes");
