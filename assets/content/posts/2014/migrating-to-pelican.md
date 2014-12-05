title: Migrating to Pelican
date: 2014-03-08
tags: website, python, flask, linux
category: blog
slug: migrating-to-pelican
author: Nikita Pekin
summary: Migrating my Flask website to Pelican and GitHub Pages

Initially this was going to be a post about deploying my newly created website to Dokku. 

However, while I was contemplating the ins and outs of the migration, I decided that because my site relied so little on the dynamic features that the [Flask]() framework provided, I decided to instead meet the following goals:

*  Migrate my site to a static platform
*  Simplify the uploading process
*  Host the site somewhere cheap and reliable

Criterion teo is really just an extension of three, so we'll consider those to be merged, giving us two real problems:

* Platform
* Host

## Platform ##

The first goal was solved by researching various blogs and simply searching for static python blogging platforms. I quickly found Alex Raichev's [website](http://raichev.net/), and I liked it quite a bit. As it was stated - quite obviously - at the bottom of the page, the site ran on [pelican](http://docs.getpelican.com), a blogging platform that was already the frontrunner of my search.

I took a look at the source of his site, but unfortunately there didn't seem to be any references to the source templates or content, only the compiled files. I did decide to settle with pelican though (or at least give it a shot), and give it a shot I did.

I created a basic website with pelican's website generator, as per their [docs](http://docs.getpelican.com/en/3.3.0/getting_started.html#installing-pelican):

    virtualenv venv --python=python2.7
    source venv/bin/activate
    pip install pelican Markdown
    pelican-quickstart

After digging around in the documentation some more, I decided to just start modifying config files and tweaking things until I achieved a workable result. It took me a while to realize that the theme of the website is structured separately from the content, so I copied the pelican [simple](https://github.com/getpelican/pelican/tree/master/pelican/themes/simple) theme into my project directory and began modifying those templates.

So I tweaked and tweaked, until I finally managed a semi-decent replica of my Flask-based homepage. When I reached that point, I began to migrate all of my pre-existing Markdown pages to the slightly different pelican format - all in all a painless process.

## Host ##

I'd previously hosted websites using [GitHub Pages](http://pages.github.com/), specifically for my high school's FIRST Robotics team - [MaxTech 4343](http://4343.ca/). After some deliberation, I decided that hosting on GitHub would fit all of my requirements - cheap (GitHub Pages is free), reliable (99.9919% uptime over the past month), and simple to upload to (done with a `git commit && git push`).

However, the deployment process wasn't quite as easy as it appeared at first glance as the `master` branch would display the website, but I also wanted to store the source in the same repository.

After a quick round of googling, I came upon a magical tool - [ghp-import](https://github.com/davisp/ghp-import), aka GitHub Pages Import. It's essentially a python module, which, when installed in conjunction with pelican, allows the developer to simply run `make github` on a branch, and the website source would be compiled with pelican and put on the `gh-pages` branch. It was exactly the tool I needed, but with some minor configuration necessary.

First I installed it:

    pip install ghp-import

I then had to move the source to another branch, as the compiled site should reside on `master`:

    git checkout -b source

I also had to update the `Makefile` to reflect these new changes:

    github: publish
        ghp-import -b master $(OUTPUTDIR)
        git push origin master

After all of these simple steps, I now had a functioning pelican site on GitHub Pages.
