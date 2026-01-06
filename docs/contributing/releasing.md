# Releasing

This page covers the release process including version management, changelog updates, and documentation versioning.

## Version Numbers

### Locations

Version numbers are maintained in two files:

| File | Format | Example |
|------|--------|---------|
| `pyproject.toml` | PEP 440 | `2.0.0`, `2.0.0.dev0`, `2.1.0a1` |
| `Cargo.toml` | SemVer | `2.0.0`, `2.0.0-dev`, `2.1.0-alpha.1` |

### Development Versions

During development, use pre-release suffixes:

```toml
# pyproject.toml
version = "2.1.0.dev0"

# Cargo.toml
version = "2.1.0-dev"
```

### Version Progression

```
2.0.0.dev0  →  2.0.0a1  →  2.0.0b1  →  2.0.0rc1  →  2.0.0
    ↓
(after release, bump to next dev)
    ↓
2.1.0.dev0  →  ...
```

## Changelog

The changelog follows [Keep a Changelog](https://keepachangelog.com/) format.

### During Development

Add entries to the `[Unreleased]` section as changes are made:

```markdown
## [Unreleased]

### Added
- New feature X

### Changed
- Modified behavior Y

### Fixed
- Bug fix Z

### Breaking Changes
- API change requiring migration
```

### At Release Time

1. Rename `[Unreleased]` to `[X.Y.Z] - YYYY-MM-DD`
2. Add a new empty `[Unreleased]` section at the top

## Documentation Versioning

Documentation is versioned using [mike](https://github.com/jimporter/mike). Each release gets its own documentation version.

### Aliases

| Alias | Purpose |
|-------|---------|
| `latest` | Points to most recent stable release (default for users) |
| `dev` | Points to development/pre-release docs |

### Deploying Documentation

**Deploy dev docs** (from main branch, pre-release):

```bash
mike deploy 2.0.0-dev dev --push
```

**Deploy a release** (updates `latest` alias):

```bash
mike deploy 2.0.0 latest --update-aliases --push
```

**Set default version** (one-time or when changing default):

```bash
mike set-default latest --push
```

**List deployed versions:**

```bash
mike list
```

**Delete a version:**

```bash
mike delete 2.0.0-dev --push
```

## Release Checklist

### Pre-Release

- [ ] All tests pass (`./check`)
- [ ] CHANGELOG `[Unreleased]` section has all changes documented
- [ ] Documentation builds without errors (`mkdocs build --strict`)

### Release

1. **Update version numbers:**
   ```bash
   # pyproject.toml: version = "X.Y.Z"
   # Cargo.toml: version = "X.Y.Z"
   ```

2. **Update CHANGELOG:**
   - Rename `[Unreleased]` to `[X.Y.Z] - YYYY-MM-DD`
   - Add new `[Unreleased]` section

3. **Commit and tag:**
   ```bash
   git add -A
   git commit -m "Release vX.Y.Z"
   git tag vX.Y.Z
   git push origin main --tags
   ```

4. **Deploy documentation:**
   ```bash
   mike deploy X.Y.Z latest --update-aliases --push
   ```

5. **Build and publish to PyPI:**
   ```bash
   ./build-release
   # Upload wheel from target/wheels/
   ```

### Post-Release

1. **Bump to next dev version:**
   ```bash
   # pyproject.toml: version = "X.Y.Z+1.dev0"
   # Cargo.toml: version = "X.Y.Z+1-dev"
   ```

2. **Commit:**
   ```bash
   git commit -am "Begin X.Y.Z+1 development"
   git push
   ```

