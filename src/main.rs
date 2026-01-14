use std::string;

use dioxus::{html::g::mask_content_units, prelude::*};
use chrono_tz::Tz;
use chrono::{DateTime, Datelike, Months, NaiveDate, Timelike, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/counter")]
    Counter { },
    #[route("/tempconv")]
    TempConv {},
    #[route("/zonetimes")]
    ZoneTimes {},
    #[route("/calendar")]
    Calendar {}
     
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {

        // Favicon in head (add <head> section or use layout)
        link { rel: "icon", href: "{FAVICON}" }
        
        // CSS stylesheets
        link { rel: "stylesheet", href: "{TAILWIND_CSS}" }
        link { rel: "stylesheet", href: "{MAIN_CSS}" }
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        div {
            class: "mx-30",
            Router::<Route> {}
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            class: "text-center",
            h2 {
                class:"text-3xl mb-6",
                "Home"
            }
            div {
                class: "mt-4 text-xl text-gray-200 flex flex-col gap-1",
                p { "This is a Demo app using Dioxus, a Rust framework." },
                p { "Dioxus supports both ", strong {class:"text-gray-100","desktop"}, " and ", strong {class:"text-gray-100","web"}," development." },
                p { "Dioxus has in-built ", strong {class:"text-gray-100","routing"}," and ", strong {class:"text-gray-100","signal"}, " feature."}
            }
            p { 
                "Try the ", 
                a { 
                    class: "text-blue-400 hover:text-blue-300 text-xl", 
                    href: "/calendar", 
                    "Calendar Demo" 
                }, 
                " now!" 
            }
        }
    }
}

/// Counter page
#[component]
pub fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx! {
        div {class:"flex flex-col gap-8",
            id: "counter",
            // Content
            h2 {
                class:"text-3xl",
                "Counter"
            }
            div {
                class:"text-4xl mx-8",
                "{count()} | {count()}", sup {small{"2"}} " = {count()*count()} | " 
                "{count()}", sup {small{"3"}} " = {count()*count()*count()}" 
            }
            div {
                class: "flex items-center gap-2",
                button {
                    onclick: move |_|{
                        count.set(count()-1)
                    },
                    class:"cursor-pointer bg-gray-300 text-gray-700 p-1 px-2 rounded-sm w-10 hover:bg-gray-200", "-",

                }
                button {
                    onclick: move |_|{
                        count.set(count() + 1)
                    },
                    class:"cursor-pointer bg-gray-300 text-gray-700 p-1 px-2 rounded-sm w-10 hover:bg-gray-200",
                    "+"
                }
            }
        }

    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            class: "mb-4 flex items-center gap-1",
            div {class:"text-4xl  mr-12", "Sample Dioxus App"}
            Link {
                to: Route::Home {},
                span {class:"bg-blue-200 p-1 text-blue-800 hover:bg-blue-500 hover:text-blue-200", "Home"}
            }
            Link {
                to: Route::Counter {},
                span {class:"bg-blue-200 p-1 text-blue-800 hover:bg-blue-500 hover:text-blue-200", "Counter"}
            }
            Link {
                to: Route::TempConv {},
                span {class:"bg-blue-200 p-1 text-blue-800 hover:bg-blue-500 hover:text-blue-200", "Temperature-converter"}
            }
            Link {
                to: Route::ZoneTimes {},
                span {class:"bg-blue-200 p-1 text-blue-800 hover:bg-blue-500 hover:text-blue-200", "Zone-times"}
            }
            Link {
                to: Route::Calendar {},
                span {class:"bg-blue-200 p-1 text-blue-800 hover:bg-blue-500 hover:text-blue-200", "Calendar"}
            }
        }

        Outlet::<Route> {}
    }
}

#[derive(PartialEq, Clone)]
struct Monthcal {
    mo: u32,
    yr: i32
}

impl Monthcal {
    fn new(y:i32,m: u32) -> Self {
        Self { yr: y, mo: m }
    }
    fn startweekday(&self) -> u32 {
        NaiveDate::from_ymd_opt(self.yr, self.mo, 1)
        .unwrap_or_else(|| NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
        .weekday()
        .num_days_from_sunday() as u32
    }
    
    fn lastday(&self) -> u32 {
        let next_yr = self.yr + (self.mo == 12) as i32;
        let next_mo = if self.mo == 12 { 1 } else { self.mo + 1 };
        NaiveDate::from_ymd_opt(next_yr, next_mo, 1)
            .unwrap_or_else(|| NaiveDate::from_ymd_opt(2026, 1, 1).unwrap())
            .pred_opt()
            .unwrap()
            .day()
    }
    
    fn cgrid(&self) -> Vec<String> {
        let first_weekday = self.startweekday();
        let mut grid = vec!["*".to_string()];
        grid.extend((1..first_weekday).map(|_| format!("*")));
        grid.extend((1..=self.lastday()).map(|d| format!("{:2}", d)));
        grid.resize(42, "*".to_string());

        grid
    }
}

#[component]
fn Calendar() -> Element {
    let months = ["Jan","Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let weekdays = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let mut curmo = use_signal(|| 0u32);
    let mut curyr = use_signal(|| 2026i32);
    // let dt = use_signal(||Monthcal::new(*curyr.read(), *curmo.read()));
    let dt = use_memo({
        let curmo = curmo.clone();
        let curyr = curyr.clone();
        move || Monthcal::new(*curyr.read(), *curmo.read())
    });
    let grid = use_memo(move || dt.read().cgrid());
    let curmonth = use_memo(move || {
        let month_num = *curmo.read();
        months.get((month_num as usize).saturating_sub(1)).unwrap_or(&"??").to_string()
    });

    rsx! {
        div {
            h1 { 
                class: "text-3xl my-4", 
                div {
                    class:"flex mr-24 p-1 px-2",
                    span {
                        class:"mr-12",
                        "Calendar :: {curmonth.read()} {*curyr.read()}"
                    },
                    nav {
                        button {class:"bg-blue-400 mx-2 p-1 px-2 rounded-sm",
                            onclick: move |_| {
                                curyr.set(curyr() - 10)
                            },
                            "|<" 
                        }
                        button {class:"bg-blue-400 mx-2 p-1 px-2 rounded-sm",
                            onclick: move |_| {
                                curyr.set(curyr() - 1)
                            },
                            "<" 
                        }

                        button {class:"bg-blue-400 mx-2 p-1 px-2 rounded-sm",
                            onclick: move |_| {
                                curyr.set(curyr() + 1)
                            },
                            ">"
                        }
                        button {class:"bg-blue-400 mx-2 p-1 px-2 rounded-sm",
                            onclick: move |_| {
                                curyr.set(curyr() + 10)
                            },
                            ">|" 
                        }
                    }
                }
            }
            div {
                class: "grid grid-cols-12 gap-2",
                for (i, m) in months.iter().enumerate() {
                    button {
                        class: "bg-blue-600 text-white cursor-pointer rounded-sm py-2",
                        onclick: move |_| {
                            curmo.set((i+1) as u32);
                        },
                        "{m}"
                    }
                }
            }
            div {
                class: "my-2 grid grid-cols-7 gap-2",
                for w in weekdays {
                    span {
                        class:"bg-blue-400 p-2 text-center",
                        {w.to_string()}
                    }
                }
            }
            div {
                class: "grid grid-cols-7 gap-1",
                for d in grid.read().clone() {
                    span {
                        class: "bg-gray-600 text-center w-full py-4",
                        "{d}"
                    }
                }
            }
        }
    }
}

#[component]
fn TempConv() -> Element {
    let mut temp = use_signal(|| 0.0f64); 
    let mut ctype = use_signal(|| "C");

    let convert = move |cty:String| -> String {
        if cty == "C" {
            let totemp = (temp() * 1.8) + 32.0;
            format!("{:.2} C = {:.2} F", temp(),totemp)
        } else {
            let totemp = (temp() - 32.0) / 1.8;
            format!("{:.2} F = {:.2} C", temp(),totemp)
        }
    };
    rsx! {
        div {
            class:"flex flex-col gap-8",
            h2 {
                class:"text-3xl",
                "Temperature Converter"
            }
            div {
                class: "flex gap-2",
                "Enter value: ",
                input {
                    class: "border-1 w-30 text-right rounded-sm p-1 px-2",
                    r#type: "number",
                    value: temp(),
                    onchange: move |ev| {
                        if let Ok(val) = ev.value().parse::<f64>() {
                            temp.set(val);
                        }
                    }
                }
                button {
                    class: "w-10 bg-gray-300 rounded-sm text-2xl text-blue-700 px-2",
                    onclick:move|_| {
                        temp.set(temp() - 1.0)
                    },
                    "-"
                }
                button {
                    class: "w-10 bg-gray-300 rounded-sm text-2xl text-blue-700 px-2",
                    onclick:move|_| {
                        temp.set(temp() + 1.0)
                    }, "+"
                }
            }
            div {
                class: "flex gap-2",
                button {
                    onclick: move |_| {
                        ctype.set("C");
                    },
                    title: "(N * 9/5) + 32",
                    class: "bg-gray-300 hover:bg-gray-200 cursor-pointer rounded-sm p-1 px-2 text-gray-600",
                    "Celcius to Fahrenheit"
                }
                button {
                    onclick: move |_| {
                        ctype.set("F")
                    },
                    title: "(N - 32) * 5/9",
                    class: "bg-gray-300 hover:bg-gray-200 cursor-pointer rounded-sm p-1 px-2 text-gray-600",
                    "Fahrenheit to Celcius"
                }
            }
            div {
                class: "text-3xl",
                {convert(ctype().to_string())}
            }
        }
    }
}

#[component]
fn AnalogClock(time: chrono::DateTime<Utc>, zone_name: String) -> Element { 
    let tz: Tz = zone_name.parse().unwrap();  // "Asia/Kolkata" → Tz
    let local = time.with_timezone(&tz);       // Utc → Tz
    let time_str = local.format("%H:%M").to_string();
    let anghr = (local.hour() as f64 * 30.0 + local.minute() as f64 / 2.0) - 90.0;
    let angmi = (local.minute() as f64 * 6.0 + local.second() as f64 / 10.0) - 90.0;

    rsx! {
        div {
            class: "text-3xl font-mono",
            hr {}
            div {
                svg {
                    view_box: "-50 -50 100 100",
                    width: 400,
                    circle { r: 49, fill: "navy" },
                    circle { r: 10, fill: "lightcyan" },
                    line { x1: "-6", x2: "32", stroke: "cornflowerblue", stroke_width: 2, stroke_linecap: "round", transform: format!("rotate({anghr})") },
                    line { x1: "-6", x2: "42", stroke: "cornflowerblue", stroke_width: 2, stroke_linecap: "round", transform: format!("rotate({angmi})") },
                    text { x: 0, y: -15, fill: "orange", font_size: 8, text_anchor: "middle", "{time_str}" }
                }
            }
        }
    }
}


#[component]
fn ZoneTimes() -> Element {
    let mut zone = use_signal(|| Tz::Asia__Kolkata);  // IST default
    let mut time = use_signal(|| Utc::now());
    let zones = [
        ("Asia/Kolkata", "India"),
        ("America/New_York", "US-East"),
        ("America/Los_Angeles", "US-West"),
        ("Europe/London", "UK"),
        ("Europe/Paris", "France"),
        ("Europe/Moscow", "Moscow"),
        ("Asia/Tokyo", "Japan"),
        ("Australia/Perth", "Aus-West"),
        ("Australia/Sydney", "Aus-East"),
        ("Pacific/Auckland", "New Zealand"),
    ];
    let zone_display = move || {
        let local = time().with_timezone(&zone());
        format!("{} {}", 
            local.format("%H:%M:%S"), 
            zone().name()
        )
    };

    rsx! {
        div { class: "flex flex-col gap-8",
            h2 { class: "text-3xl", "🌍 Zonal Times" }
            // Zone selector + steppers (your DNA)
            div {
                class: "flex gap-2",
                div { class: "flex flex-col gap-2",
                    select {
                        class: "p-1 px-2 bg-white text-gray-800",
                        value: zone().to_string(),
                        size: zones.len(),
                        onchange: move |ev| {
                            let tz_name = ev.value();
                            if let Ok(tz) = tz_name.parse() {
                                zone.set(tz);
                            }
                        },
                        for (zone_name, display) in &zones {
                            option {value: "{zone_name}", "{display}"}
                        }
                    }
                    // Live tick (your reactive magic)
                    button { class: "w-10 bg-gray-300 text-blue-700 rounded px-2", onclick: move |_| time.set(Utc::now()), "↻" }
                }
                // Live display (your converter pattern)
                div {
                    class: "col-span-2 text-4xl font-mono p-4 bg-gray-100 text-blue-700 rounded-lg", 
                    "{zone_display()}",
                    AnalogClock {time: time(), zone_name: zone().name()}
                }
            }
        }
    }
}
/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[post("/api/echo")]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
