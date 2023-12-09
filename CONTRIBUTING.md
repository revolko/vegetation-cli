# Contributing
Your contribution is very much welcomed. The most important thing is to document your changes.
Ideally, by making self-documentary code or by providing some comments.

## Workflow
 1. Fork the repository and clone:
    * you need to setup `upstream` repository, use this command: 
        
         `git remote add upstream git@github.com:revolko/vegetation-cli.git`

 2. Create commits (better to create more commits than less) and push them to a new branch

 3. Create a Pull Request (must contain only pushed commits -- no merge commits allowed)

 4. Check the pipeline results

 5. In case of an build or test error fix it

 6. If reviewer request changes, amend existing commits (creat new commits only when necessary)

 7. Rebase merge strategy is used so after merge, you need to update your master:
    * `git fetch upstream && git rebase upstream/master`
