# About

This is an experiment in [photojournalism](https://en.wikipedia.org/wiki/Photojournalism), consuming the news of the day primarily as images, rather than text.

It is a kaleidescope of the world right now, an instagram without the narcissism.

# Architecture &amp; Design

This is an [RSS feed](https://en.wikipedia.org/wiki/RSS) parser, written in [Rust](https://www.rust-lang.org/), using a simple [shared memory construct](https://tokio.rs/tokio/tutorial/shared-state) for its internal "database".

The images are presented as direct links from their sources, and are not stored beyond their availability in the live feed, nor are they altered or processed in any way.

All credits and IP ownership remain with their respective owners.

In addition to the web view, which is based on the [boostrap album](https://getbootstrap.com/docs/5.3/examples/album/), there is a rudimentary API for fetching the next batch of images from a given starting point.

The site does not issue cookies, nor is there any type of authentication required.

# Getting Started

## Build and run as a local binary

- [Install Rust](https://www.rust-lang.org/tools/install) and clone this repo
- Set the required `PHOTOJOURNALISM_` environment variables ([confg.toml](config.toml) has appropriate defaults)
- Build and run:
```sh
cargo run
```

Opening a browser to `http://0.0.0.0:9000/` (or whatever value you used for `PHOTOJOURNALISM_SERVER`) should result in an album view of the first set of current photos.

You can also confirm using the two API endpoints:

- `/health` is a health check that also returns a summary of the currently loaded content

```sh
curl http://0.0.0.0:9000/health
{
    "feeds": 10,
    "photos": 272
}
```

- `/api/next/{start_at_index}` returns a list of `NewsPhoto` structs (the actual number of results produced depends on the value of the `PHOTOJOURNALISM_PAGE_SIZE` environment variable)

```sh
curl http://0.0.0.0:9000/api/next/0 | python -m json.tool
[
    {
        "image_url": "https://static01.nyt.com/images/2023/11/23/multimedia/23finland-border-kmbp/23finland-border-kmbp-mediumSquareAt3X.jpg",
        "story_url": "https://www.nytimes.com/2023/11/23/world/europe/finland-russia-border-migrants.html",
        "description": "Finnish border guards escorting migrants at the international crossing with Russia near Salla, Finland, on Thursday.",
        "credit": "Jussi Nukari/Lehtikuva, via Associated Press"
    },
    {
        "image_url": "https://static01.nyt.com/images/2023/11/23/multimedia/23themorning-lead-promo/23themorning-lead-bmhq-mediumSquareAt3X.jpg",
        "story_url": "https://www.nytimes.com/2023/11/23/briefing/thanksgiving-pep-talk.html",
        "description": "A Thanksgiving Pep Talk",
        "credit": "Johnny Miller for The New York Times"
    }
]
```