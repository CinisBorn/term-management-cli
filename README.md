# Description 

The Term Management CLI is a small project. I started using it when I began learning English. I had a problem: How could I remember the words I learned? It was easy at first because most of the words stuck in my mind. After a few years, however, I had trouble revising words, especially the most difficult ones, such as "take," which has many definitions. So, I created this small project. 

With the Term Management CLI Project, I can add a term, access it whenever I want, read a brief description, etc. You can manage your terms stored in a local database (SQLite, for example). 
> I choose "term" instead of "word" because I want to save and revise other "terms", like "compile system". So it is indeed for broad use.

# How to install 

Copy the project: 
```bash
git clone https://github.com/CinisBorn/term-management-cli.git
```
Install `Rust` in the oficial site 
> this step will be option in future updates
Once you installed `Rust`, in the directory of project, run:
```shell
cargo build --release
```
Done! You can create an alias for the executable located in the `target` folder inside of `release`, or run it directly running in your shell: `./term-definition-management`. 

# How to use: 

There are a bunch of cli commands. I will list all command in detail below. I will use `tdm` followed by the command because it is my alias. But you can use any alias for it, or run the binary directly. 

## add 
You can add terms running the `add` 
```bash
tdm add
```
It will ask for your input. You must to specify the `term` and `more_information` field. You can edit these fields later. 
> `more_information` is optional. 
## update 
you can update all field from a term with the command `update`. It will ask for the same fields from `add` command. If you don't want to update some field, press "enter" (the input will be empty, so it will be skipped).
```bash
tdm update
```
## check 
It will check if the term already exists. 
```bash
tdm check term
```
## remove 
It will remove a term from database (once removed, it is unrecoverable. 
```bash
tdm remove term
```
## relation 
It will add a relation between two terms. It will ask for three fields: `from` term to `term` relation, and the `relation` description between them. 
```bash
tdm relation
```
## relations 
It will list all relations for a specific term. 
```bash
tdm relations term
```
## get: 
It will get all information about the term 
```bash
tdm get term
```
# Project status:
It's a personal project for me, and I will update it and fix issues in my free time. I have a roadmap in mind, but I'm inexperienced at creating projects. I will study a lot, refine it, and then create a realistic roadmap.
