# Project Maintainers

This file provides documentation for common tasks performed by project maintainers. It's intended to serve as an introduction for new maintainers, and to serve as memory for the project about how some things are currently done.

The current project maintainers are (alphabetically) Gem Lucas, James Wakefield, Jo Whiteley, Johan Frisk, Julien Comte, Laura Rettig, Matt Pearce, and Sean Stangl.

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

## How-To: Review Merge Requests

Pending merge requests can be found [on the Merge Requests page](https://gitlab.com/openpowerlifting/opl-data/merge_requests).

The general workflow is:

1. Pick a merge request.
2. Select the "Changes" tab.
3. Read through the whole merge request and make sure it makes sense. Please check every file, even the originals!
4. Leave a review, comments, or give approval.
5. If approved, wait for tests to pass (near-mandatory)!
6. If tests pass, rebase (if necessary) and merge.
7. Add a thank-you comment :-).

This will be explained in greater detail below.

### Reviewing Changes

In the "Changes" tab, you will see all the changes that the patch author is proposing. Please read through all of them and make sure that they are correct and intentional.

You can leave comments on specific lines. To do that, hover your mouse cursor over the line in question. A speech bubble will appear to the left of the line. If you click it, a text area will appear for you to leave a comment.

There are two ways to leave a comment: "Start a review" and "Add comment now." Please use "Start a review" -- it will tie all of your comments together into one bundle. The "Add comment now" is intended more for throwaway comments, whereas "reviews" are for requesting changes.

#### Common errors to look for

There are several common errors that tests don't catch, that are worth checking for:

1. Sometimes the `original.txt` contains thousands of lines of nothing but commas. Please look to make sure that the file doesn't do that.
2. Sometimes the `meet.csv` year doesn't match the year implied by the folder name. For example, `1801` but the `meet.csv` says `2003-12-02`.

### Merging a Valid Request

On a high level, the way to merge a merge request is:

1. Click the blue "Approve" button.
2. Click the green "Rebase" button. (There may also just be a green "Merge" button: if so, click that and you're done!)
3. Wait for a message about "added commits" to appear in the team chat.
4. Refresh the merge request page.
5. A blue button will appear near where the "Rebase" button was. Click the arrow to the right of it, and select "Merge Immediately".

#### What's Rebasing?

In git, every change (called a "commit") remembers its history, which is an ordered list of all the commits that preceded it. So for example, if you make a change `A`, then a change `B`, then `C`, the change `C` remembers that its parent is `B`, and `B` remembers that its parent is `A`.

That looks like this, as a list: `A <= B <= C`. We call `C` the "head" of the "branch".

Now suppose someone made a merge request, and that merge request is `C` like above. Merge requests take a while to have tests run. In the meantime, I come along and I make some changes to the main branch, and I push a commit named `D`.

Because `C` wasn't merged yet, the most recent commit on the main branch was `B`. So `D` remembers that its parent was `B`. On the main branch, that looks like: `A <= B <= D`.

So now we have a conflict: you can't have both `A <= B <= C` and `A <= B <= D`!

There are two ways to solve this. The way we use is called "rebasing", which means that we rewrite the history of the merge request to basically lie about where it came from, so that it makes sense linearly.

So when you hit that green "Rebase" button, what it's doing is telling `C` that its parent is now actually `D`. It does that by replaying the changes on top of the main branch.

So when you hit the Rebase button, the merge request changes to `A <= B <= D <= C`.

At that point, GitLab recognizes that the merge request (`A <= B <= D <= C`) is linearly compatible with the main branch (`A <= B <= D`), and so it gives you an option to do a "fast-forward merge", which just means that it adds `C` to the main branch.

Sometimes the green "Rebase" button can fail. That happens mostly if the merge request is old (like, a day) -- which is why we try to get to them quickly. Sometimes also people forget to give you the ability to click that button, and then it's just missing and has to be resolved on the command-line.


## Using Git

It is not necessary for maintainers to be familiar with the git command-line, although unfortunately it is necessary in certain circumstances to resolve merge request conflicts.

So the following is just bonus extra-credit material.

### Disallowing merge commits

This project uses a linear history via "squashing" and does not use what git calls "merges". Unfortunately, merging is the default behavior in git, and so it's very easy to accidentally cause a merge.

The best way to prevent this is to open up your `.git/config` file and add the line `mergeoptions = --ff-only` to the main branch. For example, in my config:

```
[branch "main"]
        remote = origin
        merge = refs/heads/main
        mergeoptions = --ff-only
```

If you then use `git merge` on a branch that isn't just a fast-forward of the main branch, instead of adding a merge commit, it will merely complain.

### Fetching merge requests

You can use `git fetch` to also fetch all merge requests. Then you can check them out directly using `git checkout origin/merge-requests/5000`, for example. This is a very, very handy shorthand.

To get that set up, add the `merge-requests` line to your `.git/config` in the `[remote "origin"]` section:

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
