
//! # Datastore fields
//!
//! `mddb` uses several components of a markdown file to create fields:
//!
//! - Path (full path of file)
//! - File name (just the file name)
//! - Base name (just the file name, without the extension)
//! - Hierarchy (relative to the base directory in the connect string)
//!   - a list of the levels in a hierarchy
//! - Domain (top-level of hierarchy)
//! - Creation date
//! - Modification date
//! - Hash / Checksum of file
//! - Front matter
//! - Headings
//! - Check boxes (GFM task lists)
//!
//! ## Notes
//!
//! - [ ] How do we handle notes with no id?
//! - [ ] Is it ok to add an id to a file? (configurable?)
//! - [ ] how do we create a "global" configuration that all functions can use.
//!   for example, a function needs to know the `base directory`
