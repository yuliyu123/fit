# fit
Fast git tool to manage and sync your develop branch with upstream easily and fastly.

## alias git cmd

`fit create <branch>`: git branch new_branch & git checkout new_branch

`fit checkout <branch>`: git checkout master

At branch:

`fit pull`: git pull upstream master, pull the latest code from upstream master branch.

`fit push`: git push --set-upstream origin new_branch.

`fit rebase`: git pull upstream master --rebase, pull the latest code from upstream master branch to rebase.

`fit delete`: git push origin --delete new_branch & git branch -D new_branch: delete both local and remote branch.

`fit sync`: sync fork rep, git fetch upstream -> git checkout master -> git merge upstream/master

`fit list`: git branch -a
