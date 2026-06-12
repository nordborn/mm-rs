# mm-rs

Simple HFT bot in Rust (top-level-actors implementation)

It's intentilonally is not fully implemented (strategy stub, some methods of exchange connectors are stubs too), but demonstrates actor-based concurrency approach via WS market data subscribtion and processing.

--

This exact implementation (top-level-actors) is made very closer to well-known actor-based concurrency approach when actors are globally visible and any of them can be accesses via actor registry, the working app is actor-awared supposes that all necessary actors started on startup and can be called from any point of the app. To hide message passing, the code provides facades for the actors. 

For polymorphism (including stub, mocks) we inject dependensies into exact actor and then call it as usually, from any point.

This is how actor model implemented on Erlang/Elixir (globally exposed and accessible actors with registry), but, I'd say it has pros and cons from modern 
point of view of projects architechure.

The pro/cons reasons are mainly due to the hard global relaying on top-level visibility of the actors and close to a global Service Locator pattern: simple way to call dependency, but hard way to understand execution graph. These tradeoffs, IMO, forces to use this approach for little-to-middle projects but brings significant complexity for large projects. Meanwhile, good documentation may solve the issue. 

Thus, the DI-based approach is shown in the `di-actors` branch, when actor is hidden under its adapter structure implementing necessary trait and can be considered as more welcome for general purpose app building.

In any case, using of global top-level actors eliminates any thougths of `Send + Sync` constraints (because facade functions of an actor are just pure static functions), and allows to write any multithreaded Rust code thinking of its dependencies like it would be singlethreaded, allows to spawn what your want when you need without fear of getting stuck with Rust constraints (because actors are not bounded with any lifetime of a caller and so on).

So, it's up to you whuch approach to use: `top-level-actors` or `di-actors. Just consider using actors to build large-scale and fast concurrent applications to achieve clarity, flexibility and extensibility.

## OBSERVATION

    cargo install hotpath --features tui
    hotpath console