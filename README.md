# mm-rs
Simple HFT bot (demo)

The repo has 2 independent branches and demonstrates 2 slightly different approaches to write concurrent code in Rust using actor-based model.

> NOTE: Actor is an async runtime object that uses so called Mailbox (think channel) for message passing (for request acccepting) safely from many threads instead of direct method calls, then it processes each message on by one and, if necessary, answers (sends the response) to the caller via channels too. So, an actor's internally processing code is singlethreaded and avoids multithreaded complexity while the actor itself is safe to use in mutlithreaded programs and can be used as a building brick (it's invented for that actually). In actor systems it also usually has a Name and the actor system usually provides a Registry to get access to actors by their names. Optionally, actor systems may provide distribution but this is out of the current scope.

So, pls peek `top-level-actors` and `di-actors` to compare the approahes and patterns, their pros and cons and code base organization and choose the most suitable for your app.

`top-level-actors` demonstrates global-actor-aware architecture, while `di-actors` wraps and hides actors as dependencies. However, both utilize the benefits of the actor-based model.