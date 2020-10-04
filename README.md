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
