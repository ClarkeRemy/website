use std::path::{Path, PathBuf};

use maud::{html, Markup, PreEscaped};
use serde_json::json;
use ssg::{Asset, Source, Targets};

use crate::html::css_class;
use crate::mobs::FullCalendarEvent;
use crate::style::{BUTTON_CLASSES, BUTTON_GAP, TEXT_COLOR};
use crate::url::Url;

pub(crate) fn js_library_asset() -> Asset {
    Asset::new(PathBuf::from("/fullcalendar.js"), async {
        Source::Http(
            Url::parse("https://cdn.jsdelivr.net/npm/fullcalendar@6.0.2/index.global.min.js")
                .unwrap()
                .to_inner()
                .clone(),
        )
    })
}

pub(crate) fn markup(
    targets: &Targets,
    events: Vec<FullCalendarEvent>,
    display_event_time: bool,
) -> Markup {
    #[derive(Debug, PartialEq, Eq)]
    enum Direction {
        Left,
        Right,
    }
    fn arrow(direction: Direction) -> Markup {
        let mut classes = classes!("w-[1em]" format!("fill-{TEXT_COLOR}"));
        if direction == Direction::Right {
            classes.push("rotate-180".parse().unwrap());
        }
        html! {
            svg
               xmlns="http://www.w3.org/2000/svg"
               viewBox="0 0 32 32"
               class=(classes) {
                   path d="M13,25.6c-0.5,0-1-0.2-1.4-0.6l-8.3-8.3c-0.4-0.4-0.4-1,0-1.4L11.6,7c0.6-0.6,1.4-0.7,2.2-0.4c0.8,0.3,1.2,1,1.2,1.8V12h12 c1.1,0,2,0.9,2,2v4c0,1.1-0.9,2-2,2H15v3.6c0,0.8-0.5,1.5-1.2,1.8C13.5,25.5,13.3,25.6,13,25.6z" {}
                               }
        }
    }
    const CALENDAR_FN_SNIPPET: &str = include_str!("pages/calendar.js");
    let calendar_container_class = css_class();
    let date_range_class = css_class();
    let timezone_class = css_class();
    let button_prev_class = css_class();
    let button_next_class = css_class();
    let button_today_class = css_class();
    let calendar_fn_input = json!({
        "events": events,
        "displayEventTime": display_event_time,
        "selectors": {
            "calendarContainer": format!(".{calendar_container_class}"),
            "dateRange": format!(".{date_range_class}"),
            "timezone": format!(".{timezone_class}"),
            "buttonPrev": format!(".{button_prev_class}"),
            "buttonNext": format!(".{button_next_class}"),
            "buttonToday": format!(".{button_today_class}"),
        },
    });

    const INPUT_ATTR: &str = "data-input";
    let input_selector = format!("[{INPUT_ATTR}]");

    html! {
        div class=(classes!("flex" "justify-between" "items-center" "flex-wrap" format!("gap-x-{BUTTON_GAP}"))) {
            div class=(classes!("flex" "flex-wrap" "gap-x-[1ch]" "whitespace-nowrap" "flex-1")) {
                p class=(classes!(timezone_class)) {}
                p class=(classes!(date_range_class)) {}
            }
            div class=(classes!("flex" format!("gap-x-{BUTTON_GAP}"))) {
                div class=({BUTTON_CLASSES.clone() + classes!(button_prev_class)}) {
                    (arrow(Direction::Left))
                }
                div class=({BUTTON_CLASSES.clone() + classes!(button_next_class)}) {
                    (arrow(Direction::Right))
                }
                button class=({BUTTON_CLASSES.clone() + classes!(button_today_class)}) { "Today" }
            }
        }
        div class=(classes!(calendar_container_class "[--fc-page-bg-color:transparent]")) {}
        script defer src=(targets.path_of(Path::new("/fullcalendar.js")).unwrap()) {}
        script data-input=(calendar_fn_input.to_string()) {
            (PreEscaped(format!("window.addEventListener('DOMContentLoaded', () => {{
                const input = JSON.parse(document.querySelector('{input_selector}').getAttribute('{INPUT_ATTR}'))
                {CALENDAR_FN_SNIPPET}(input)
            }})")))
        }
    }
}
