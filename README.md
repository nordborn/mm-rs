# mm-rs

Simple HFT bot in Rust

It's intentilonally is not fully implemented (strategy stub, some methods of exchange connectors are stubs too), but demonstrates actor-based concurrency approach via WS market data subscribtion and processing.

This exact implementation (DI-first) primary made with clean architecture in mind when the invoking code is not actor-awared and only compostion root (in main function) selects concrete implementations of traits based on actors:
- the actors are represented with their adaptors as strust implementing appropriate trait(s)
- the adaptors can be used as dependensies, they are generally simple structs just keeping their names and maybe some clonable/copyable config params
- the adaptors hide message passing, can be cloned (due to their simple nature) and their async methods can be invoked

Comparing to top-level-actors approach (when we suppose that the actors are globally visible and we allow call those global actors from any part of the code - this is how Erlang/Elixir implements it), this code have some limitations (again because it avoids calling to globally-exposed actors) like necessarity to embed the depencies affecting some behavior flexibility, but brings other benefits like very straightforward dependency graph and execution graph and, of course, all DI benefits. So, it's up to you which approach to choose.


## OBSERVATION

    cargo install hotpath --features tui
    hotpath console