First of all, thank you for your interest in contributing to the Atros project. It's nice to see you among us.

The best way to find what to contribute in the project is to use it personally for your different needs. To do that, install Atros in your system. You can find the [Getting Started guide in the README file](/README.md#getting-started), follow these steps to start contributing.

## About the Repositories

The main repository is the one you're inside currently, and it's a single binary Rust application that follows Rust's module structure. Therefore, it's simple to work on.

There are two more repositories to know about: [config template](https://github.com/Atros-Automation-Tools/config-template), which is the main configuration template that you clone to your .config/ directory when you run `atros init`, and in this template repository, we have [config-prerequisites](https://github.com/Atros-Automation-Tools/config-prerequisites) as a git submodule. The reason for having a separate repo is to be able to update Atros easily without wiping users' defined steps. Since the type definitions in that repository might get updated at some point, when we implement an `upgrade` CLI command in this tool, we want to upgrade not only the binary itself but also the prerequisites folder (which is .atros in the ~/.config/atros/ folder)

## How to Contribute to the Main Repository

### Fork This Repo

You can fork this repo by clicking the fork button in the top right corner of this page.

```sh
# Clone on your local machine
git clone git@github.com:your-username/atros.git

# Navigate to the project directory
 cd atros/

# Create a new branch
git checkout -b cool-branch
```

Then, you're ready to hack!

```sh
cargo run               # Will print help text
cargo run -- <command>  # Will run the command you provide: init, run, help, etc.
```

## How to Contribute to Prerequisites

If you want to change type definitions or add new ones, this will be helpful.

### Fork the Repo

Go to [config-prerequisites](https://github.com/Atros-Automation-Tools/config-prerequisites) repository, and fork it.

### Replace the default prerequisites with yours

```sh
# Go to your config
cd ~/.config/atros

# Remove the default prerequisites folder
rm -rf .atros

# Clone your forked repo
git clone git@github.com:your-username/config-prerequisites.git .atros

# Navigate to the repo
cd .atros

# Create a new branch
git checkout -b cool-types
```

And again, you're ready to hack!

## Commit Convention

Before you create a Pull Request, please check whether your commits comply with the commit conventions used in this repository.

When you create a commit, we kindly ask you to follow the convention `category(module or subject): message` in your commit messages while using one of the following categories:

- feat / feature: All changes that introduce completely new code or new features
- fix: Changes that fix a bug (ideally you will additionally reference an issue if present)
- refactor: Any code-related change that is not a fix or a feature
- docs: Changing existing or creating new documentation (i.e. README, type documentation)
- build: All changes regarding the build of the software, changes to dependencies, or the addition of new dependencies
- test: All changes regarding tests (adding new tests or changing existing ones)
- ci: All changes regarding the configuration of continuous integration (i.e. GitHub Actions, CI system)
- chore: All changes to the repository that do not fit into any of the above categories

Examples:

- feat(testing): get user input to skip succeeded tests
- feat: bash file support for writing tests
- refactor(testing): change in cache's JSON structure to make it more comprehensive

If you are interested in the detailed specifications, you can visit https://www.conventionalcommits.org/
