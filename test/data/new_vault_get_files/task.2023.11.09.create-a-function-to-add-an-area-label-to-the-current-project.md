---
id: x5zrpcotlrb55tunyre42qk
title: Create a Function to Add an Area Label to the Current Project
desc: ''
updated: 1699566837446
created: 1699566012350
status: ''
due: ''
priority: ''
owner: ''
---


The current convention is to have github labels that cover:

- `area.` - Areas : Components or major features of the project, including the build process, testing , etc.
- `closed.` - "Closed reasons" : a tag for why the particular issue was closed.
- `flag.` - cross-functional tags : such as excluding it from an automation process, etc.
- `is.` - the type of issue :  these are an approximation of the conventional commit types
- `sp.` - Story points : the story point value
- `wf.` - Waiting for : This issue needs something such as a review

In addition to these, I have some basic colors set for the "heat" of the label.  Cooler labels are blues and greens, while yellow -> orange -> red denote levels of heating up

The areas have two types of blue: `bfd4f4` is a light blue and these represent "generic" areas, ones that should reasonably apply in every project.. such as `area.build`, etc.; and `2564b9` which is a project specific
"component" area.  So:

```powershell
`Set-IssueLabel` -area 'componentA' -description 'Related to the componentA component'
```

Would create a "dark blue" label called `area.componenta`
