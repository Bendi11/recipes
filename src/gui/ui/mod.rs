pub mod recipe;
pub mod search;
pub mod home;

use druid::{
    widget::{Flex, ViewSwitcher},
    LensExt, Widget, WidgetExt,
};

use crate::gui::{
    theme,
    widgets::{
        icon::{self, Icon},
        separator::Separator,
    },
};

use super::{
    data::{
        screen::AppScreen,
        search::{Query, SearchState},
        AppState,
    },
    GOLDEN_RATIO,
};

pub fn root_widget() -> impl Widget<AppState> {
    let sidebar = Flex::column()
        .with_spacer(theme::SPACING)
        .with_child(
            Icon::svg(&icon::BOWL_ICON)
                .with_scale(10.)
                .with_color(theme::COLOR_4)
                .fix_size(150., 150.),
        )
        .with_child(Separator::new(5.).with_ratio(1.))
        .with_default_spacer()
        .with_child(
            search::search_bar().lens(AppState::search.then(SearchState::query).then(Query::term)),
        )
        .with_default_spacer()
        .with_flex_spacer(0.5)
        .padding((5., 0., 0., 0.));

    let screen = Flex::row()
        .with_child(sidebar.fix_width(200.))
        .with_default_spacer()
        .with_child(Separator::new(5.).vertical(true).with_color(theme::COLOR_2))
        .with_default_spacer()
        .with_flex_child(
            ViewSwitcher::new(
                |state: &AppState, _env| state.screen,
                |screen, _state, _env| match screen {
                    AppScreen::Home => home::home_widget().boxed(),
                    AppScreen::SearchResults => search::search_screen().boxed(),
                    AppScreen::View => recipe::view_screen().boxed(),
                },
            )
            .expand(),
            GOLDEN_RATIO * 1.7, //Is it still the golden ratio, just... scaled?
        );

    screen
}
