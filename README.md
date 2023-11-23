# About

This is an experiment in [photojournalism](https://en.wikipedia.org/wiki/Photojournalism), consuming the news of the day primarily as images, rather than text.

It is a kaleidescope of the world right now, an instagram without the narcissism.

# Architecture &amp; Design

This is an [RSS feed](https://en.wikipedia.org/wiki/RSS) parser, written in [Rust](https://www.rust-lang.org/), using a simple [shared memory construct](https://tokio.rs/tokio/tutorial/shared-state) for its internal "database".

The images are presented as direct links from their sources, and are not stored beyond their availability in the live feed, nor are they altered or processed in any way.

All credits and IP ownership remain with their respective owners.

In addition to the web view, which is based on the [boostrap album](https://getbootstrap.com/docs/5.3/examples/album/), there is a rudimentary API for fetching the next batch of images from a given starting point.

The site does not issue cookies, nor is there any type of authentication required.