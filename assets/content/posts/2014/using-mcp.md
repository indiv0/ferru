title: Using MCP
date: 2014-08-24
tags: minecraft, mcp
category: blog
slug: using-mcp
author: Nikita Pekin
summary: Using MCP.
status: draft

# Prerequisites

* Python 2
* wine

# Setup

Download the recommended `src` build from [here](http://files.minecraftforge.net/).

Extract the archive.

Run `./gradlew setupDecompWorkspace --refresh-dependencies`.

Now run `./gradlew eclipse`.

To run the server, use `./gradlew runServer`.

To run the client, use `./gradlew runClient`.

Ensure that the server you are connecting to is either in offline mode, or pass in your username to the client with `--username USERNAME --password PASSWORD`.
