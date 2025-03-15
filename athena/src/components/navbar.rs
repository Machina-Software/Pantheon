use dioxus::prelude::*;

use dioxus_free_icons::icons::fa_brands_icons::FaGithub;
use dioxus_free_icons::icons::fa_solid_icons::FaBars;
use dioxus_free_icons::Icon;

use crate::Route;

#[component]
pub fn Navbar(show_sidebar: Signal<bool>) -> Element {
    // we have to use use_signal for this as i'm not sure to apply css
    // to Icons direct
    let mut sidebar_toggle_hover = use_signal(|| false);
    let mut github_hover = use_signal(|| false);

    rsx! {
        div {
            class: "bg-zinc-950 h-16 flex items-center justify-between border-0 border-b-2 border-gray-600",
            div {
                class: "flex flex-row items-center w-64",
                button {
                    class: "cursor-pointer p-4",

                    onmouseenter: move |_event| {
                        sidebar_toggle_hover.set(true);
                    },
                    onmouseleave: move |_event| {
                        sidebar_toggle_hover.set(false);
                    },
                    onclick: move |_event| {
                        let new_value = !show_sidebar.read().clone();
                        *show_sidebar.write() = new_value;
                    },
                    Icon {
                        width: 28,
                        height: 28,
                        fill: if *sidebar_toggle_hover.read() {"white"} else {"lightgray"},
                        icon: FaBars,
                    }
                }
                div {
                    class: "flex flex-row border-2 border-gray-600 items-center justify-center ml-1",
                    div {
                        class: "w-full h-full",
                        img {
                            class: "ml-2 mb-1",
                            src: asset!("assets/cdo-logo.png"),
                        }
                    }
                    div {
                        class: "flex flex-col p-2 m-1",
                        Link {
                            class: "text-gray-300 hover:text-white font-sans text-4xl",
                            to: Route::Home {},
                            "Athena"
                        }
                        h1 {
                            class: "text-gray-400 font-sans text-sm",
                            "v0.0.1"
                        }
                    }
                }
            }
            div {
                class: "w-64 grow h-full flex flex-row gap-0 justify-left items-left",
                div {
                    class: "p-2 border-r-2 border-gray-600 flex flex-col w-24 h-full items-center",
                    div {
                        class: "text-gray-300 text-md",
                        "Tartarus"
                    }
                    div {
                        class: "text-gray-400 text-sm",
                        "192.168.1.2"
                    }
                }
                div {
                    class: "p-2 border-r-2 border-gray-600 flex flex-col w-20 h-full items-center",
                    div {
                        class: "text-gray-300 text-md",
                        "Uptime"
                    }
                    div {
                        class: "text-gray-400 text-sm",
                        "100 hours"
                    }
                }
            }
            div {
                class: "h-full p-2 border-l-2 border-gray-600",
                a {
                    href: "https://github.com/Dack985/Pantheon",
                    target: "_blank",
                    onmouseenter: move |_event| {
                        github_hover.set(true);
                    },
                    onmouseleave: move |_event| {
                        github_hover.set(false);
                    },
                    Icon {
                        class: "w-full h-full",
                        fill: if *github_hover.read() {"white"} else {"lightgray"},
                        icon: FaGithub,
                    }
                }
            }
        }
    }
}
