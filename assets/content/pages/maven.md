Title: Maven
Date: 2014-03-02
Slug: maven
Author: Nikita Pekin
Summary: An overview of my Java CI and maven process

I use [Jenkins CI](http://ci.nikitapek.in/) for the continuous integration of my java plugins.

The CI automatically polls my Github repositories every minute for the latest commits, compiles them with maven, then deploys the artifacts to my repository.

Via this process, the latest builds of all of my plugins can be found in my maven [repository](#repository), which is hosted on Amazon AWS.

Repository
==========

I maintain a maven repository which hosts all of my maven projects.

If you wish to include one of my plugins in your project, you can do so by adding this repository to your `pom.xml`:

    <project>
      ...
      <repositories>
        ...
        <repository>
          <id>indiv0's Repo</id>
          <url>http://repo.nikitapek.in/maven/releases</url>
        </repository>
        ...
      </repositories>
      ...
    </project>
