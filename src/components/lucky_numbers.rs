use crate::components::button::Button;
use crate::components::switch::{Switch, SwitchThumb};
use dioxus::prelude::*;

const FACT_WIN_TWO_OF_THREE: u8 = 2;
const FACT_WIN_THREE_OF_THREE: u8 = 5;

fn get_random_i8() -> Result<u32, getrandom::Error> {
    Ok(getrandom::u32()?)
}

pub fn update_lucky_nums_array() -> [u8; 3] {
    let mut numarr: [u8; 3] = [1, 2, 3];
    for i in 0..numarr.len() {
        let mut n: u8 = 0;
        while n == 0 {
            n = (get_random_i8().unwrap() % 10) as u8;
            numarr[i] = n
        }
    }
    numarr
}
fn update_lucky_nums_set(num_array: [u8; 3]) -> std::collections::HashSet<u8> {
    std::collections::HashSet::from([num_array[0], num_array[1], num_array[2]])
}
fn calculate_win_loss(num_set: std::collections::HashSet<u8>, money_in: u32) -> u32 {
    if num_set.len() == 2 {
        info!("2 numbers equal");
        money_in * (FACT_WIN_TWO_OF_THREE as u32)
    } else if num_set.len() == 1 {
        info!("3 numbers equal");
        money_in * (FACT_WIN_THREE_OF_THREE as u32)
    } else {
        0
    }
}
fn update_balance(balance: u32, amount_win: u32, money_in: u32) -> u32 {
    balance + amount_win - money_in
}
#[allow(non_snake_case)]
#[component]
pub fn LuckyNumbersTable() -> Element {
    let iter_nums: [u8; 3] = [0, 1, 2];
    let mut lucky_numbers: Signal<[u8; 3]> = use_signal(|| [1, 2, 3]);
    let mut num_set: Signal<std::collections::HashSet<u8>> =
        use_signal(|| std::collections::HashSet::from([1, 2, 3]));
    let mut balance: Signal<u32> = use_signal(|| 100);
    let mut money_in: Signal<u32> = use_signal(|| 10);
    let mut amount_win: Signal<u32> = use_signal(|| 0);
    let mut checked1 = use_signal(|| false);
    let mut checked2 = use_signal(|| false);
    let mut checked3 = use_signal(|| false);
    let mut chk = use_signal(|| [checked1, checked2, checked2]);

    rsx! {
        div {
            div {
                id: "table_account_info",
                "margin": "auto",
                "width": "300px",
                div { "Balance: {balance}"}
                div { "Money In: {money_in}"}
            }

            div {
                "class":"container",
                id: "lucky_nums",
                "width": "300px",
                for (name, mut i) in lucky_numbers().iter().zip([checked1, checked2, checked3]) {
                    div {
                        Switch {

                            checked: i(),
                            // aria_label: "Switch Num {num }",
                            on_checked_change: move |new_checked| {
                                i.set(new_checked);
                            },
                            SwitchThumb {}
                        }
                        br {}
                        "\n{name}"
                    }
                }
            }
            if balance() > 0 {
                div {
                    id: "button_test",
                    Button {
                        id: "button_play",
                        onclick: move |_mouse_event| {
                            lucky_numbers.set(update_lucky_nums_array());
                            num_set.set(update_lucky_nums_set(lucky_numbers()));
                            amount_win.set(calculate_win_loss(num_set(), money_in()));
                            balance.set(update_balance(balance(), amount_win(), money_in()));
                        },
                        "Play!"
                    }
                }
            }
            else {
                p { "You lost!" }
            }
        }
    }
}
