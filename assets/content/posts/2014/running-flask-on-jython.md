title: Running Flask on Jython
date: 2014-02-05
tags: website, linux, jython, python, java
category: blog
slug: running-flask-on-jython
author: Nikita Pekin
summary: Short version for index and feeds
status: draft

The [MinerApocalypse](http://www.reddit.com/r/minerapocalypse/) plugin development team (pretty much myself and [/u/ams2990]()) are building a block-logging suite for use on the MinerApocalypse server.

One of the components of the suite is a web-interface which integrates with the database to which the plugin provides data. This interface is then used to browse and modify logs over the internet, without having to manually edit the database, or enter Minecraft to enter commands.

Initially, [Apache Tomcat]() was going to be used to provide the interface, but the initial code was cluttered, and a collection of various fixes and patches.

It was then decided that the interface would be re-written with [Jython](), with one of the major Python web frameworks at its core, to provide the service. After some investigation, it was discovered that [Flask]() is not supported on Jython, and [Django]() has not been supported since version 1.3.

In third place, [CherryPy]() was the next choice.

Initially, the Jython v2.5.4-rc1 Traditional Installer was downloaded from the Jython [releases]() page.

Jython was then installed with `java -jar jython-installer-2.5.4-rc1.jar` into `~/bin/jython2.5.4rc1/`. The jython executable was then symlinked to `/usr/local/bin/` with `ln -s ~/bin/jython2.5.4rc1/jython /usr/local/bin/`.

pip installation of CherryPy failed, so ultimately CherryPy was installed with easy_install:

    wget http://peak.telecommunity.com/dist/ez_setup.py
    jython ez_setup.py
    ~/bin/jython2.5.4rc1/bin/easy_install CherryPy

However, CherryPy failed to run, because of `easy_install` issues. So instead, we returned to Flask. Manually installing Flask and all of its dependencies with the following commands showed that Flask could indeed run under Jython:

    wget https://bitbucket.org/pypa/setuptools/raw/bootstrap/ez_setup.py
    jython ez_setup.py install

    wget https://pypi.python.org/packages/source/M/MarkupSafe/MarkupSafe-0.18.tar.gz
    wget https://pypi.python.org/packages/source/J/Jinja2/Jinja2-2.7.2.tar.gz
    wget https://pypi.python.org/packages/source/W/Werkzeug/Werkzeug-0.9.4.tar.gz
    wget https://pypi.python.org/packages/source/i/itsdangerous/itsdangerous-0.23.tar.gz
    wget https://pypi.python.org/packages/source/F/Flask/Flask-0.10.1.tar.gz
    tar -xvf *

Install each of these by entering their directories and running the command `jython setup.py install` in their respective directories.

After creating a basic Flask site, the following workaround needed to be added to the main site module:

    :::python
    # workaround for Jython, see: http://bugs.jython.org/issue1521
    import __builtin__
    if not hasattr(__builtin__, "buffer"):
        def _buffer(object, offset=None, size=None):
            if offset is None:
                offset = 0
            if size is None:
                size = len(object)
            return object[offset:offset + size]
        __builtin__.buffer = _buffer

Run with `jython site.py`.
