# Creating a Release

[GitHub](https://github.com/orhun/runst/releases) and [crates.io](https://crates.io/crates/runst) releases are automated via [GitHub actions](.github/workflows/cd.yml) and triggered by pushing a tag.

1. Run the [release script](./release.sh): `./release.sh v[X.Y.Z]` (requires [git-cliff](https://github.com/orhun/git-cliff) for changelog generation)
2. Push the changes: `git push`
3. Check if [Continuous Integration](https://github.com/orhun/runst/actions) workflow is completed successfully.
4. Push the tags: `git push --tags`
5. Wait for [Continuous Deployment](https://github.com/orhun/runst/actions) workflow to finish.
