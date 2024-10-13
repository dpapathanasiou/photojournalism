# About

This is an experiment in [photojournalism](https://en.wikipedia.org/wiki/Photojournalism), consuming the news of the day primarily as images, rather than text.

It is a kaleidescope of the world right now, an instagram without the narcissism.

# Architecture &amp; Design

This is an [RSS feed](https://en.wikipedia.org/wiki/RSS) parser, written in [Rust](https://www.rust-lang.org/), using a simple [shared memory construct](https://tokio.rs/tokio/tutorial/shared-state) for its internal "database".

The images are presented as direct links from their sources, and are not stored beyond their availability in the live feed, nor are they altered or processed in any way.

All credits and IP ownership remain with their respective owners.

In addition to the web view, which is based on the [boostrap album](https://getbootstrap.com/docs/5.3/examples/album/), there is a rudimentary API for fetching the next batch of images from a given starting point.

The site does not issue cookies, nor is there any type of authentication required.

An example is now up and running at [saruzai.com](https://www.saruzai.com/) (see the section below on how to host your own instance).

# Getting Started

## Build and run as a local binary

- [Install Rust](https://www.rust-lang.org/tools/install) and clone this repo
- All the tests should pass: `cargo test`
- Set the required `PHOTOJOURNALISM_` environment variables ([confg.toml](config.toml) has appropriate defaults) and run: `cargo run`

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
curl http://0.0.0.0:9000/api/next/0
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

## Building the docker image

Use the [Dockerfile](Dockerfile) to create and run this application in a container; in addition to [docker](https://www.docker.com/get-started/), this code and instructions have been confirmed to work under [Rancher Desktop](https://rancherdesktop.io/) as well:

```sh
DOCKER_BUILDKIT=1 docker build --tag photojournalism --file Dockerfile .
```

## Running the image

```sh
docker run -p 9000:9000 \
-e PHOTOJOURNALISM_SERVER='0.0.0.0:9000' \
-e PHOTOJOURNALISM_PAGE_SIZE=6 \
-e PHOTOJOURNALISM_FETCH_INTERVAL=3600 \
-e PHOTOJOURNALISM_FEED_LIST='/app/feeds.txt' \
-e PHOTOJOURNALISM_STATIC_PATH='/app' \
-e RUST_BACKTRACE=1 \
-e RUST_LOG='debug' \
--name photojournalism-container \
photojournalism
```

note that the host defined in `PHOTOJOURNALISM_SERVER` *must* be `0.0.0.0` otherwise connecting from outside the container fails.

## Confirming

Opening a browser to `http://0.0.0.0:9000/` or running these commands from outside the container should result in successful responses:

```sh
curl http://0.0.0.0:9000/health
curl http://0.0.0.0:9000/api/next/0
```

## Debugging

Attach to the running image:

```sh
docker exec -it photojournalism-container /bin/sh
```

## Host your own instance

All you need are the bundle of files in the [static](static) folder, and a self-contained executable file, which you can create by running `cargo build --release` (the single `photojournalism` binary file gets built in the `target/release` folder).

Set the required `PHOTOJOURNALISM_` environment variables ([config.toml](config.toml) has appropriate defaults) to match your filesystem layout before running it.

Optionally, you can use something like [systemd](https://www.baeldung.com/linux/systemd-services-environment-variables) on linux to run it as a service, so that it starts automatically on system start and reboots.

If you host it under your own domain, a proxy service such as [nginx](https://nginx.org/), along with free SSL certificates from [Let's Encrypt](https://letsencrypt.org/) are useful add-ons.

[Digital Ocean](https://www.digitalocean.com/) is a particularly good hosting provider to consider, especially given their [excellent tutorials](https://docs.digitalocean.com/products/) for all of the above (using [this link](https://m.do.co/c/71387faa5599) to sign up gets you $200 in credit, and I gain a small referral credit as well).


# Contribute

[Pull requests](https://help.github.com/articles/about-pull-requests/), for either improving the [code](src), or adding to the [list of feeds](feeds.txt) (or both) are welcome!

In particular, if you come across (yet) another feed which provides images differently than what the [parser](src/parser.rs) can currently process, please consider taking a snapshot, adding it to the [test fixtures](tests/fixtures), and updating the parsing logic, with a corresponding [test case](src/parser_test.rs).