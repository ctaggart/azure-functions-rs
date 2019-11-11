// WARNING: This file is regenerated by the `cargo func new` command.

mod call_join;
mod join;
mod looping;
mod say_hello;
mod select;
mod start;
mod start_looping;
mod timer;

// Export the Azure Functions here.
azure_functions::export! {
    call_join::call_join,
    join::join,
    say_hello::say_hello,
    start::start,
    looping::looping,
    start_looping::start_looping,
    timer::timer,
    select::select,
}
