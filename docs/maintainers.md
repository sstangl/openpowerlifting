# Project Maintainers

This file provides documentation for common tasks performed by project maintainers. It's intended to serve as an introduction for new maintainers, and to serve as memory for the project about how some things are currently done.

The current project maintainers are (alphabetically) Gem Lucas, Jo Whiteley, Matt Pearce, and Sean Stangl.

## What's a Project Maintainer?

A Project Maintainer is someone who has push access to our GitLab project repository. In [the GitLab project members listing](https://gitlab.com/openpowerlifting/opl-data/-/project_members), that means their "role permission" is set to "Developer" ([see the permissions documentation](https://docs.gitlab.com/ee/user/permissions.html#project-members-permissions)).

Those permissions allow maintainers to change the repository: to add new changes, and review and merge any merge requests.

## What does a Project Maintainer do?

Pretty much what they were doing before, but also:

### 1. Keeping the tests passing.

Since maintainers are the only people who can make changes, it is their responsibility to make sure that those changes are correct and won't break things. The problem with breaking things is mostly that it affects new merge requests: they will fail for reasons that have nothing to do with the changes in the request itself, and it will be harder to merge.

The way to check the passed/failing status of the project is [by using the Pipelines page](https://gitlab.com/openpowerlifting/opl-data/pipelines) on GitLab.

If there is a failure, you can click the "failed" button to look at the output and see what needs fixing.

The absolute best way to keep the tests passing is to only merge things that have passed the tests already! Failures only happen when we merge things without checking.

### 2. Reviewing merge requests.

Healthy projects need to respond to issues and merge requests quickly, because quick turnaround makes the project more fun for new contributors. Ideally, we would give some sort of response within a few hours. The requests don't have to actually be merged: we just need to acknowledge them, either by approving them or by commenting on any changes that may be necessary.

### 3. Making good commit messages.

The value of using git is that every file is given a linear history. We can look at that history and see what changes we made to that file over time.

For example, suppose we would want to look up all the changes made to `uspa/1630`. From the project root, I could do that by executing `git log --pretty=oneline --abbrev-commit -- meet-data/uspa/1630/entries.csv`, which shows:

```
a509832774 Specify Jesus Hernandez 1 in uspa/1630 and add his IG. Closes #4181
aca0e2a2b0 Clean up WPF config.
494f128d56 Add 2019 WPF World Cup & configure WPF. #4046.
ed9d85f1f1 Disambiguate Marissa Mendoza. Closes #4120
64717f7dfc Add 2019 USPA Monger Mayhem
```

All of those changes affected that file. Now, the problem is that by default, the commit message is something completely unhelpful, like "Update entries.csv". Imagine if all those messages just said "Update entries.csv" -- we'd have no idea what they did!

Project Maintainers have the ability to change commit messages before merging something.

A good commit message is formatted like this:

`{description of the change} {where the change was made}`

The description of the change must always begin with an imperative verb, like "Add", "Disambiguate", "Specify", "Rename", etc. So for example, please say `Add 2019 USAPL Texas Championship` instead of `USAPL Texas Championship`.

## Making Mistakes

Maintainers will make mistakes. The good news is that with git, it is very easy for us to fix mistakes, either by reverting the change or by fixing it directly. It is inevitable that mistakes will be made. When that happens, please don't worry about blame, and instead ask the following:

1. Is there some way we could add tests or other systems that would prevent the mistake again in the future?
2. Could we add checklist-style documentation somewhere to make sure that we remember to avoid the cause of the mistake?

## Using Git

It is not necessary for maintainers to be familiar with the git command-line, although unfortunately it is necessary in certain circumstances to resolve merge request conflicts.

So the following is just bonus extra-credit material.

### Disallowing merge commits

This project uses a linear history via "squashing" and does not use what git calls "merges". Unfortunately, merging is the default behavior in git, and so it's very easy to accidentally cause a merge.

The best way to prevent this is to open up your `.git/config` file and add the line `mergeoptions = --ff-only` to the master branch. For example, in my config:

```
[branch "master"]
        remote = origin
        merge = refs/heads/master
        mergeoptions = --ff-only
```

If you then use `git merge` on a branch that isn't just a fast-forward of the master branch, instead of adding a merge commit, it will merely complain.

### Fetching merge requests

You can use `git fetch` to also fetch all merge requests. Then you can check them out directly using `git checkout origin/merge-requests/5000`, for example. This is a very, very handy shorthand.

To get that set up, add the `merge-requests` line to your `git/config` in the `[remote "origin"]` section:

```
[remote "origin"]
        url = git@gitlab.com:openpowerlifting/opl-data.git
        fetch = +refs/heads/*:refs/remotes/gitlab/*
        fetch = +refs/merge-requests/*/head:refs/remotes/origin/merge-requests/*
```

### Learning Git

I don't have good references at the moment, but if I find some, I will put them here.

People have trouble understanding git because they learn the commands first, and don't understand what the system is actually doing conceptually. It is a significantly better use of time to first learn the concepts, so you have an idea of what git is trying to accomplish, and only then learn the actual commands.

A good tutorial will mention "staging".

Alternatively, you can try to find a graphical frontend to git that may make things much more visually obvious, so you don't have to hold so much in your head.
