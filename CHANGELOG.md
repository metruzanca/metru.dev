# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.4] - 2026-07-03

### Added

- Meta description tag for SEO.
- `robots.txt` with sitemap reference.
- `lang="en"` attribute on `<html>` element.

### Fixed

- Heading hierarchy: landing contact section `h3` → `h2`.
- Heading hierarchy: How I Work inspiration links `h4` → `h3`.
- Heading hierarchy: added missing `h1` page titles to Projects and Blog pages.

## [0.1.3] - 2026-06-30

### Added

- How I Work page with principles, process notes, tooling, collaboration guidelines, and inspiration links.
- About page with personal story, work timeline, LinkedIn references, side projects, community involvement, and a collapsible crypto exit note.
- Shared `utils/datetime` module for date formatting reused across pages.

### Changed

- Reordered nav links: About, How I Work, Projects, Blog, Resume, Music.
- Landing page: removed numbered section headings, reordered sections (Projects first), replaced button CTA with inline sentence links.
- Contact, Git Activity, and Music sections on landing page no longer render section headings.
- Resume and About pages inlined date formatting calls to shared utility module.

## [0.1.2] - 2026-06-30

### Added

- Command palette (Ctrl+K / Cmd+K) with fuzzy search across all site pages and blog posts, section headers, arrow key navigation, and auto-scroll following the selection.

## [0.1.1] - 2026-06-29

### Added

- Music page powered by Last.fm — shows what you're currently listening to, recent tracks, and scrobble stats with a live waveform visualizer.

## [0.1.0] - 2026-06-29

### Added

- Synthwave design system.
- Landing page featuring:
  - Hero section.
  - Projects from GitHub (pinned repos, contribution graph).
  - Blog posts section with hover-expand.
  - Contact section with real links and obfuscated email.
- Resume page with GitHub Gist data and print layout.
- Umami analytics tracking.
- Open Graph and Twitter Card meta tags for social link previews.
- Calendar scheduling link in contact section.
