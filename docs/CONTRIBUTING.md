# Contributing

Thanks for your interest in contributing to Tartan Vote!

Before contributing to this repository, please discuss the change you wish to make via issue on this repository, email to one of the codeowners, or on the ScottyLabs [discord](go.scottylabs.org/discord).

## How Can I Contribute?

For now, please just refer to the communication channels listed above. As this project matures, we will establish a more well-formed contributing structure.

## Documentation

When making a change, it would be wonderful if you could update the corresponding documentation. If you cannot or are unsure how to, please leave an issue or let [Yiyoung Liu](github.com/maybe-yiyi) know so that the documentation does not lag behind. If the documentation does not exist, don't worry about it! (or write the documentation yourself, that would be greatly appreciated.)

## Pull Requests

Direct pushes to main are blocked. You should create a branch (if you are a contributor in ScottyLabs) or fork the repository, make your changes, then create a PR to main.

## Style Guide

- All Rust code should be formatted using `cargo fmt` and linted with `cargo clippy`. The CI/CD will check that all PR'ed code passes `cargo fmt` and `cargo clippy`.
- All Svelte code should checked with `deno run check`. The CI/CD will automatically check this too.

### Commit Guidelines

I am a firm believer in the [kernel commit style](https://docs.kernel.org/process/submitting-patches.html). Not all of those sections are useful, such as the fact that we do not mail patches (unfortunately), but most of those pieces of advice are helpful nonetheless. Good commit habits reflect on the developer. Being able to clearly reflect upon your changes and describe the impact of them means you are able to reason about your code and about why you are making the changes you are.

#### Commit Subjects

Commit subjects should be styled in the following method:

```
system: subsystem (if applicable): short description
```

A list of possible commit types, but not exhaustive:

- `backend: auth: created migrations for token storage`
- `backend: session: ensures user must exist before joining`
- `docs: process: add section on code review`
- `devenv: update to latest scottylabs version`
- `frontend: motion: center vote div`

I would prefer not to see 'chore: format' or 'fix: some stuff'. This is not helpful to me as a maintainer or to your future self or other people by being vague about what you are doing.

It should not be *terribly* difficult to write commit subjects. If it feels that your commit can't be easily grouped into a system or subsystem, perhaps reevaluate if you should split your commit into two or more smaller commits.

#### Commit Descriptions

In addition, add a description to your commits. This is where you summarise the changes you made and why you made them, so that anyone can come back and read about the thought process and reasoning behind the changes.

You can more easily write a long commit description with the command `git commit` rather than `git commit -m`.

The description should truncate lines at about 80 characters (it should do this automatically if you are editing via command line, but I'm not too sure about other editors). This makes it easier to read commits on terminal screens from `git log` and on the git repos.

#### Making fixes

Perhaps I will ask you to make some changes to your code. While it is tempting to make your fixes and make a commit called `fixes`, I recommend against you doing that, and rebasing your changes into the commit in which it goes along with.

For example, say (hypothetically) I get a PR submitted to me, with some changes that look like

```diff
--- a/main.c
+++ b/main.c
...
+ printf("hi
```

Now, you may not need to know how to read a patch file, and you may not know how to read c code the best, but you can probably tell that that code probably doesn't compile (it's missing a quotation mark, a parenthesis, and a semicolon!). A lazy way to fix this code would be to make a new commit called `main: fix syntax`, but when I merge your changes people don't really want to see that you fixed some syntax in the git history...

The best (and in my opinion, correct) way to do this is to rebase your commits. You can make a random commit message (doesn't really matter, it'll disappear anyways) for these new fixes.

Then, you can use the command `git rebase -i HEAD~2` (or however many commits you want to go back, such as `HEAD~5`, etc.) to bring up the interactive rebasing screen.

When you change the word in front of your newest commit to `fixup` or `f`, for example

```
pick a943d2e main: print hi message
pick 27eaa11 random commit message
```

turns into

```
pick a943d2e main: print hi message
fixup 27eaa11 random commit message
```

saving and leaving the file will combine your fixup commit with the one above it, and this cleans up your git history! Now you can `git push --force` to update your PR upstream. (don't worry, pushing with force to your own branch is OK, but don't do it to others without their approval!)

#### Undoing fixes

Maybe you messed up. That's perfectly fine! Git provides you many tools to undo your mistakes.

> [!NOTE]
> This documentation will be updated soon.
