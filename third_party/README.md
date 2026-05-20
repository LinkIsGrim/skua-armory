This directory contains third-party vendored modules tracked as git submodules.

Currently expected submodule:
- `third_party/arma-rs` — fork at https://github.com/LinkIsGrim/arma-rs on branch `fix/loadout_into_arma`

When cloning the repository, initialize submodules:

```bash
git submodule update --init --recursive
```

CI must also initialize submodules (see workflow edits).
