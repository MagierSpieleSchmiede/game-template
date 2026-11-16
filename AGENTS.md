# AGENTS.md — start here

You are working on a game built from **Magier SpieleSchmiede's game template**: a game in Rust with Bevy, whose entire gameplay is a **deterministic, headless tick simulation** in the crate `game_sim`, running in `FixedUpdate`; the presentation in `game_client` is a pure observer that writes back exactly one thing: the intent. Randomness comes from the one seeded RNG resource; all durations are ticks, not seconds. Targets: Windows, Linux, macOS, Android, iOS (web when feasible), English first (add more locales as needed), keyboard/mouse and gamepad alike.

`docs/` is the **vault**, the design memory of the project (an Obsidian vault, published as a website). Maintain it when you change something. But do **not** read it all: that costs many tokens. Read what the handover in `docs/status.md` names, and look up the rest only when the task needs it.

## Starting and ending a session

1. **Start:** read `docs/status.md`. It is the handover of the last session: what was done, the next task, which pages it needs, hints. Read exactly those pages, no more. Where code contradicts a page, the code wins; then fix the page.
2. **Work** following the flow in `docs/backlog.md` (tests first, gates green, then update vault and backlog).
3. **End:** check `target/` (below), then **overwrite** the handover in `docs/status.md`: what you did, the next task, which pages it needs, what the next session must know. Keep it short; history belongs in the milestone's implementation log.

Regardless of the task:

- Do not start writing code unprompted; the maintainer decides when implementation begins.
- Which ADRs are accepted and which are only *proposed* is listed in `docs/decisions/decisions.md`. Proposed ADRs are planning material; ask before building code on them.

## Look up when the task needs it

| Page | what for |
| --- | --- |
| `docs/index.md` | content map of the vault |
| `docs/backlog.md` | all tasks, dependencies, flow per item |
| `docs/architecture/simulation-model.md` | the invariants and the tick chain |
| `docs/specifications/spec-001-prototype.md` | toolchain, scope, requirements, acceptance criteria |
| `docs/decisions/decisions.md` | all ADRs with status; never re-open an accepted ADR without a new ADR |
| `docs/problems/problem-log.md` | open questions with their next step |
| `docs/architecture/security.md` | mandatory reading for any change touching network, external input or add-ons |
| `docs/templates/conventions.md` | how pages, ADRs and P-entries are written, which fact lives where |

## Before committing: the gates

```sh
cargo fmt --check
cargo check
cargo clippy --all-targets -- -D warnings
cargo test
```

**Testing without wasting tokens:**

- **While working**, only what the change touches: `cargo test -p game_sim`, `cargo test <name-part>`, `cargo clippy -p <crate>`. Cargo rebuilds only what changed.
- **Before committing**, all gates, but quietly: with `-q` and reading only the end of the output or the errors (e.g. `cargo test -q 2>&1 | tail -5`), never whole build logs.
- **Other platforms** (Windows, macOS, Android, iOS, wasm32) are not built locally; CI (`.github/workflows/ci.yml`) checks them on every push to `dev-*` and in pull requests. Its result is visible without login at `https://api.github.com/repos/<org>/<repo>/actions/runs`; read logs only when a job is red, and then only the red step.

**Previewing the website:** the `preview` branch is for experimenting; anyone may push to it at any time, even with `--force` (e.g. `git push -f origin HEAD:preview`). The site built from it appears after about a minute under `https://<org>.github.io/<repo>/preview/`; the main site stays untouched. It only builds when `docs/` or `site/` changed; otherwise start the *Deploy vault to GitHub Pages* action for `preview` manually. `preview` is never merged into `main`.

## Rules

- **No Bevy API from memory.** Bevy knowledge from before the current version is not authoritative. Before the first code, read the migration guides; check every uncertain API in the real crate docs or source (`~/.cargo/registry/src/`). The same applies to Avian.
- **Required Components** and simple `commands.spawn((...))` tuples. No legacy bundles.
- **Keep the simulation invariants**, in particular: no gameplay state in `Local<>`; no frame time in simulation code, all durations in ticks; randomness only from the one seeded RNG resource; ordering by `SimId`, never by `Entity`; simulation entities carry no meshes, materials, lights or UI; each gameplay quantity (damage, health, death, …) has exactly one code path.
- **The presentation writes back exactly one thing: the intent.** The simulation never reads presentation state.
- **Read only through the observation.** HUD, AI, add-ons and network sending get their data from the one observation function with a visibility filter, never directly from the ECS world.
- **Teams, not "the opponent".** No code assumes exactly two players.
- **Animation never blocks an ability.** When something takes effect is decided by the simulation in ticks; no animation events or root motion in gameplay.
- **No visible text in code.** Everything through localisation keys.
- **No key bindings in code.** Input through actions; keyboard/mouse and gamepad equal.
- **Platforms:** nothing that excludes Windows, Linux, macOS, Android or iOS; `game_sim` must build for `wasm32-unknown-unknown` (no threads, no file system, no system time). Check platforms for every new dependency (`docs/architecture/platforms.md`).
- **Balance constants** belong in the central balance resource, never as magic numbers in systems.
- **Scope:** no abstraction that does not serve the "design for" list or is used at least twice. No TODO stubs for core functionality.
- **`Cargo.lock` is committed.** New dependencies only with justification (an ADR if it is an architecture question).
- **Branches and `main`:** nobody commits directly to `main`. Everyone works on their own branch `dev-<name>` and creates it themselves; agents work on the branch of the person they work for. Everything reaches `main` via pull request; merge when the maintainer says so.
- **Never commit:** `target/`, `site/.quartz/`, `site/public/`, `.env`, `.secrets`.

## Keeping `target/` small

Bevy builds are large: a single debug build puts several gigabytes into `target/`, and every dependency version change, extra profile (`--release`, `test`, `clippy`) or extra target directory adds more. Cargo never cleans up old states by itself.

- **At the end of every session that compiled:** check the size (`du -sh target`). Over **20 GB**: `cargo clean`, and say so in the closing report.
- **After every dependency version change:** `cargo clean` once, because the old artefacts stay around otherwise.
- **No `--release`** unless the task needs it (performance measurement); afterwards `cargo clean --release`.
- **No extra target directories** (`CARGO_TARGET_DIR`).
- **Never `cargo clean` while another build is running**, and never delete directories outside the repo.

## Maintaining the vault

- **Conventions** are in `docs/templates/conventions.md`: title "topic — the point in half a sentence", front matter with `title:`, references as Obsidian links `[[filename|display text]]`.
- **One source per fact:** do not copy what another page already says; link to it. The table for that is in the conventions.
- **Decisions** get no prose, but an ADR following `docs/templates/adr-nnnn.md` and a line in `docs/decisions/decisions.md` (the next free number is listed there).
- **Open questions** get a page following `docs/templates/p-nnn.md` and a line in `docs/problems/problem-log.md` (the next free number is listed there).
- **Keep status up to date:** whoever builds something or changes the planning state updates `docs/status.md` and the milestone's implementation log.
- The vault is in English; identifiers in code are English too.
