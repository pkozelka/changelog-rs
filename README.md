# Changelog toolkit


is supposed to replace manual maintenance where possible, while still leaving the release author in control.

When cutting a new release, it might come handy to have following automated:

- **validate** Unreleased section items; failure must tell what needs to be fixed
- **add links** where possible (PRs, issues, components, authors, maybe even tag names)
- **pre-release**: replace the Unreleased section name with current tag and date
- **post-release**: auto-add the Unreleased section
- **post-release**: upload latest section to Github Release changelog
- **import**: explore commits since last release, show omitted changes(PRs) and suggest new entries on console

These form elementary actions useful during release.

## Starting with a new project

is easy:

```
changelog new
```

and it generates a `CHANGELOG.md` file, with minimalistic structure ready for adding new items.

## First time use on a live project


```
changelog init
```

This time, commit messages will be read from git, and with some effort to remove ballast and organize 
them into well-known groups.

If there are release git tags, they will be used to create version sections in the changelog. 

## Regular use

```
changelog sync
```

Updates changelog with new commit messages from git.

An attempt will be made to only add what is really missing.
Last release found in changelog will be the stopping point for git scanning.
Any commits referencing issues which are already in `Unreleased` part will be unchanged.
