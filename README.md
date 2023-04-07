# OwO what's this

Werdol is a very crummy desktop application Wordle clone. It's Wordle, but worse.

![A screenshot of werdol](screenshot.png)

This was a proof-of-concept intended to experiment with the [ECS architectural patttern](https://en.wikipedia.org/wiki/Entity_component_system) using Rust's [Bevy ECS library](https://bevyengine.org/).

In retrospect, Wordle doesn't lend itself super well to ECS. It was probably not the best introduction.

# Running / building / hacking

## With Nix

This repo is a Nix flake. Assuming you have a [Nix installation](https://nixos.org/download.html) and you've [enabled flakes](https://nixos.wiki/wiki/Flakes#Enable_flakes), you can just:

* Run `nix run github:agbrooks/werdol` to let Nix fetch this repo, reproducibly build Werdol, and run it. EZPZ.

* Clone this repo and run `nix build` to build the Werdol binary (and run it from `result/bin/werdol` if you want to play).

* Clone this repo and run `nix develop` to get a development environment with an LSP server and all the usual Rust goodies, exactly the same as the one I used. You can run `cargo build` here and everything should "just work."

* Do [all the usual handy Nix/flake-enabled stuff](https://nixos.org/manual/nix/unstable/command-ref/new-cli/nix3-flake.html).

## Without Nix

Well, the Nix way is much less of a hassle, but I guess there's no accounting for taste.

I haven't actually tried this, but here's roughly how you'd go about it:

Recent versions of Bevy require a nightly version of the Rust toolchain (at the time of writing). This is [somewhat annoying](https://bevyengine.org/learn/book/getting-started/setup/) but doable.

Beyond that, you'll need some non-Rust libraries, dev headers, and build tools. Probably `cmake`, `libudev`, `libxkbcommon`, etc. The `flake.nix` should give you a clue about which you'll need.

After that, run `cargo build` and pray.

# Caveats

## It's not a perfect clone

OK, you know what? It's not just "not perfect," it's a _very bad_ clone. I never really bothered with the word list / word validation parts of the game. Yes, those are essential parts of the Wordle game. However, my main concern was learning the graphics/input handling parts of the problem.

However, we can squint our eyes a little and pretend that this is a feature! You can ask unsuspecting friends to play it and laugh with them when the word is revealed to be random gibberish. You can also ask unsuspecting _enemies_ to play it and laugh _at_ them. Imagine the possibilities!

## Building/running on MacOS

I did all my development on Linux because that's what I know best.

This might _almost_ build on MacOS, but its dependence on platform-specific graphics libraries means that you'll probably need to tweak the `buildInputs` to swap out some of the non-Rust libraries that Bevy abstracts over.

## Building/running on Windows

good luck, have fun
