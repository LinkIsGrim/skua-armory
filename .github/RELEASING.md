# Releasing

Three workflows form the release chain:

```
Bump (manual)  →  Release (auto)  →  Publish to Workshop (auto)
```

- **Bump** (`bump-version.yml`, `workflow_dispatch`) — writes `chore: prepare build X.Y.Z.B` to master.
- **Release** (`version-bump.yml`, push to master touching `addons/main/script_version.hpp`) — validates, runs CI via `workflow_call` into `arma.yml`/`debug-check.yml`/`extension.yml`, creates the `vX.Y.Z` tag and GitHub Release.
- **Publish to Workshop** (`release.yml`, push of `v*.*.*` tag) — pre-existing, builds extension + `hemtt release` + Workshop upload.

The workflow files are checked in and self-contained. What is **not** in version control and must exist on the repo/org for this chain to work is documented below. If any of it is lost (org rebuild, repo transfer, secret rotation), this is how to put it back.

## Environment setup (recreate from scratch)

### 1. GitHub App for branch-protection bypass

Master branch is gated by a ruleset requiring PRs. The default `GITHUB_TOKEN` can't bypass it, so **Bump** authenticates as a dedicated GitHub App whose identity is on the ruleset's bypass list.

1. Register the app: *Org Settings → Developer settings → GitHub Apps → New GitHub App*.
   - Name: `Skua International` (must be globally unique; if changed, update the bypass entry and the references in this doc).
   - Homepage URL: anything (the repo URL is fine).
   - Webhook: **uncheck "Active"**.
   - **Repository permissions**: `Contents: Read & Write`. Everything else: *No access*.
   - Subscribe to events: none.
   - Where can this be installed: *Only on this account*.
2. On the app's settings page: note the **App ID** (number at the top), then **Generate a private key** — downloads a `.pem` file. The App ID is **not** the same as the installation ID.
3. Install the app: app's page → *Install App* → select the `skua-international` org → *Only select repositories* → check `skua-armory`.

### 2. Repo secrets

For the **Bump** workflow:

```bash
gh secret set APP_ID --repo skua-international/skua-armory --body <APP_ID>
gh secret set APP_PRIVATE_KEY --repo skua-international/skua-armory < /path/to/key.pem
```

For **Publish to Workshop** (pre-existing, unrelated to the bump flow, listed for completeness):

- `ARMA3_TOOLS_URL` — signed URL to an Arma 3 Tools tarball (consumed by `arma-actions/arma3-tools`).
- `STEAM_USERNAME` / `STEAM_PASSWORD` — Steam account that owns the Workshop item `3680089530` (item ID is hardcoded in `release.yml`).

### 3. Master ruleset + bypass

The canonical ruleset is checked in at `.github/master-ruleset.json`. If recreating, import it and then add the app to the bypass list (the app's actor_id is org-specific and can't be baked into the JSON):

```bash
gh api repos/skua-international/skua-armory/rulesets \
  --method POST --input .github/master-ruleset.json
```

Then in the UI: Settings → Rules → Rulesets → "master protection" → *Bypass list* → *Add bypass* → search the app name (`Skua International`) → mode `Always` → Save.

If migrating away from a pre-existing classic branch protection on master, delete it after the ruleset is in place so they don't both apply:

```bash
gh api repos/skua-international/skua-armory/branches/master/protection --method DELETE
```

## Troubleshooting

- **`Integration not found` from `actions/create-github-app-token`.** App isn't installed on the repo (check `https://github.com/organizations/skua-international/settings/installations`), or `APP_ID` secret holds the installation ID instead of the App ID. Verify against `gh api orgs/skua-international/installations` — the `app_id` field is what `APP_ID` should be; the `id` field is the installation ID.
- **`push declined due to repository rule violations` from the Bump workflow.** The app isn't on the master ruleset's bypass list. Re-add via the UI.
- **`Release` workflow doesn't fire after a manual bump.** Confirm the commit actually changed `addons/main/script_version.hpp` (path filter is exact) and landed on `master`.
- **Validation failure on a legitimate bump.** `BUILD` must increment by exactly 1 from the previous commit; `(MAJOR,MINOR,PATCHLVL)` must be strictly greater than the previous tuple. Skipping a build number is rejected on purpose.
