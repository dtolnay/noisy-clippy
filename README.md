## How often is each Clippy lint suppressed on crates.io

Clippy's most severe flaw in my experience has been low-signal lints that are
enabled by default, aren't worth resolving and commonly need to be suppressed.

I use Clippy across a large number of my Rust projects, so I already get good
visibility into which lints are misguided, buggy, or have unacceptably low true
positive rate on real world code. One of my [hobbies] apparently is deleting
such lints from Clippy or downgrading them out of the set of enabled-by-default
lints (opt-out lints) to the `pedantic` or `restriction` (opt-in) groups
instead.

This repo contains a script for analyzing suppressed lints on a bigger corpus:
all of crates.io. For every Clippy lint, the program counts how many times it is
suppressed globally (at module scope) or locally (on one single place the lint
is triggered).

In the table below, I would recommend paying attention to the **style** and
**perf** lints. Highly suppressed **style** lints indicate that the community
has consciously decided that Clippy's opinion on style is wrong. Highly
suppressed **perf** lints indicate that the community does not consider it
valuable to make their code more obtuse for the sake of questionable alleged
performance. I think it would be wise to delete or downgrade many of these.

[hobbies]: https://github.com/rust-lang/rust-clippy/pulls?q=is%3Apr+is%3Amerged+author%3Adtolnay+downgrade

<br>

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this project by you, as defined in the Apache-2.0 license,
shall be dual licensed as above, without any additional terms or conditions.
</sub>
