---
title: Markdown Database (mddb)
---


[![CICD](https://github.com/aldrichtr/mddb/actions/workflows/ci.yml/badge.svg)](https://github.com/aldrichtr/mddb/actions/workflows/ci.yml)
[![Version info](https://img.shields.io/crates/v/mddb.svg)](https://crates.io/crates/mddb)

## Overview

This crate uses a collection of markdown files as a database.  Files,
directories, frontmatter, headings and links all contribute to the data in this
database.

## Vault

There is a growing number of applications that use a directory of markdown files
with "wiki links" between them to create a [personal knowledge
base](https://en.wikipedia.org/wiki/Personal_knowledge_base) such as obsidian,
logsec, and, my personal favorite
[dendron](https://github.com/dendronhq/dendron).

A common theme in all of these programs is to call the collection of files a
`vault`.
