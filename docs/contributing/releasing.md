# Releasing

This guide covers branching strategy, CI/CD workflows, and the release process.

## Branching Strategy

| Branch | Purpose |
|--------|---------|
| `main` | Latest development code, always deployable |
| `v*` tags | Release versions (e.g., `v1.0.0`) |
| Feature branches | Short-lived branches merged to `main` |

**Workflow:**

1. Development happens on feature branches, merged to `main`
2. Releases are created by tagging commits with `vX.Y.Z`
3. Tags trigger automated builds and deployments

## CI/CD Workflows

Three GitHub Actions workflows automate the project:

### `ci.yml` — Continuous Integration

**Triggers:** All pushes and pull requests

| Job | Description |
|-----|-------------|
| **Lint** | `cargo fmt --check`, `cargo clippy` |
| **Build & Test** | Build package, run unit tests |
| **Docs** | Verify documentation builds |

### `docs.yml` — Documentation Deployment

**Triggers:** Push to `main`, version tags, manual dispatch

Builds documentation with MkDocs and deploys to GitHub Pages using GitHub Actions deployment.

### `deploy.yml` — Release Builds

**Triggers:** GitHub Release published

Builds wheels for all platforms (Linux, macOS, Windows) and Python versions (3.9–3.14), then attaches them to the GitHub Release.

## Version Numbers

Versions are maintained in two files:

| File | Format | Example |
|------|--------|---------|
| `pyproject.toml` | PEP 440 | `2.0.0`, `2.0.0.dev0` |
| `Cargo.toml` | SemVer | `2.0.0`, `2.0.0-dev` |

**Progression:**

```
2.0.0.dev0 → 2.0.0a1 → 2.0.0b1 → 2.0.0rc1 → 2.0.0 → 2.1.0.dev0
```

## Changelog

Follow [Keep a Changelog](https://keepachangelog.com/) format. Add entries to `[Unreleased]` during development, then rename to `[X.Y.Z] - YYYY-MM-DD` at release.

## Release Checklist

### 1. Pre-Release Verification

```bash
./check  # Runs lints, tests, and doc build
```

### 2. Update Version & Changelog

Edit `pyproject.toml` and `Cargo.toml`:

```toml
version = "X.Y.Z"  # Remove dev suffix
```

Update `CHANGELOG.md`:

- Rename `[Unreleased]` → `[X.Y.Z] - YYYY-MM-DD`
- Add new empty `[Unreleased]` section

### 3. Commit, Tag, and Push

```bash
git add -A
git commit -m "Release vX.Y.Z"
git tag vX.Y.Z
git push origin main --tags
```

### 4. Create GitHub Release

1. Go to **Releases → Draft a new release**
2. Select the `vX.Y.Z` tag
3. Add release notes (copy from CHANGELOG)
4. Click **Publish release**

This triggers:

- `deploy.yml` → Builds and attaches wheels
- `docs.yml` → Deploys documentation

### 5. Post-Release

Bump to next dev version:

```toml
# pyproject.toml
version = "X.Y.Z+1.dev0"

# Cargo.toml  
version = "X.Y.Z+1-dev"
```

```bash
git commit -am "Begin X.Y.Z+1 development"
git push
```
