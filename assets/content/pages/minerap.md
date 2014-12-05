Title: MinerApocalypse
Date: 2014-03-02
Slug: minerap
Author: Nikita Pekin
Summary: A FAQ/guide for the MinerApocalypse server

##Summary

MinerApocalypse is a Minecraft server which intends to present unrestricted gameplay to the player in an apocalyptic environment.

###Goal
To provide a balanced, apocalyptic, experience while remaining as light-weight (plug-in-wise) and as close to vanilla Minecraft as possible.

###Server Address(es)
The MinerApocalypse server address is **minerap.com**.
The MinerApocalypse development server, where plugin and other testing happens, can be accessed at **dev.minerap.com**.

###Notice
This guide is unofficial and still under construction, which means that it can and will change, and may be out-dated, incorrect, or incomplete. You have been warned.


##Rules

MinerApocalypse's rules are occaisionally updated and it is therefore best to get them at the [source](https://docs.google.com/document/d/1DKfLt70SRjtMEQvPMZgbfJLVRf4xfNrTTvE__YOQ-sw/edit).


##Plug-ins

###Custom
* [BlockSaver](http://dev.bukkit.org/bukkit-plugins/blocksaver/)
* [BookDupe](http://dev.bukkit.org/bukkit-plugins/bookdupe/)
* [ExpBottles](https://github.com/mickealn/expbottles/)
* MinerApocalypse
* [PearlNerf](http://dev.bukkit.org/bukkit-plugins/pearlnerf/)
* [Radio](http://dev.bukkit.org/bukkit-plugins/radio/)

###Third Party
* [CombatTag](http://dev.bukkit.org/bukkit-plugins/combat-tag/)
* [Prism](http://dev.bukkit.org/bukkit-plugins/prism/)
* [NoCheatPlus](http://dev.bukkit.org/bukkit-plugins/nocheatplus/)
* [Orebfuscator](http://dev.bukkit.org/bukkit-plugins/orebfuscator/)
* [Privileges](http://dev.bukkit.org/bukkit-plugins/privileges/)
* [ProtocolLib](http://dev.bukkit.org/bukkit-plugins/protocollib/)
* [WorldBorder](http://dev.bukkit.org/bukkit-plugins/worldborder/)


##Commands

While MinerApocalypse tends to avoid the use of commands in order to maximize gameplay immersion, there are several important commands which are useful for all players to know.

Certain commands have **aliases** which are simply alternative (usually shorter) ways to call the command.

The following commands are the most commonly used. The full list of commands can be obtained by using **/help** in-game.

###/ct
Gives the remaining status on the player's CombatLog time.

This command is used to see if it is safe to log off without leaving behind an NPC which other players can attack to obtain the player's items.

**See: Combat Tagged**

###/kill
Commits suicide for the player.

This is used when players wish to end their suffering (e.g. unavoidable starvation).

The command has a cooldown to prevent abuse of the system for rapid travel across the map.

###/ping
Determings your ping (latency) to the server.

A lower number is always better. Numbers above 400 ms generally indicate a very poor connection between you and the server.

###/unstuck
Teleports the player upwards onto the nearest stable surface

Occasionally Minecraft will glitch and cause the player to enter a solid block. This will suffocate the player.

Alias: **/!**

###/helpop
Sends a message to all mods/admins monitoring the HelpOp channel.

Used to report suspicious/illegal activity (such as cheating), or to ask mods/admins for help.

However, it is recommended to ask questions via PMs (private messages) to the mods through Reddit or by posting a thread on the [MinerApocalypse subreddit](http://www.reddit.com/r/minerapocalypse/).

###/whisper
Greatly reduces chat message distance, effectively whispering the player's message to those ~1-2 blocks around them.

Alias: **/w**

###/shout
Increases a player's chat message distance, at the cost of some hunger for the player. Allows messages to be heard twice as far.

Alias: **/s**

###/reply
Used to reply to a private message from a mod. This is usually needed when discussing a **/helpop** with a mod.

Alias: **/r**

##Plug-in Functionality

Various plug-ins on the server provide the necessary functionality for a gameplay experience on the server.

The server has local chat, which reduces the chat distance to approximately 50 blocks. After a certain distance, the message will begin to fade away, and will eventually disappear for other players.

Players can duplicate books and their contents to allow for mass-production of print materials. The [BookDupe](http://dev.bukkit.org/bukkit-plugins/bookdupe/) page on BukkitDev has further instructions.

Players can store their EXP into bottles for storage/sale by holding empty glass bottles in their hand and right clicking on an enchanting table. The amount of EXP stored in one Bottle O' Enchanting has been adjusted so that 64 bottles stores enough EXP to reach level 30 from level 0.

More information on the functionality of plugins and their relevant commands can be found on their respective pages


##FAQ

###When will the server be updated to 1.7/1.8/etc.?
The server when update when there is a recommended [Spigot build](http://ci.md-5.net/job/Spigot/) for the requested version, and when all of the plugins have been updated to support this version.

This date is **impossible to predict**, but a good rule of thumb is approximately 1.5 months after the initial release of the version.

###I just logged in. Where am I?
MinerApocalypse spawns you in a random location the first time you log in. This also occurs whenever you die, unless you have a slept in a bed, which will result in that bed being your spawn point until it is obstructed or broken.

###Does the map have a size limit? If so, what is it?
Yes it does. The map has a size limit of 3000 blocks in each direction, making the map encompass a total of **6000*6000** blocks in size, centred on Origin.

This functionality is provided by WorldBorder.

###Why are my animals suffocating in walls/fences/etc.?
This is caused by a known Minecraft bug, [MC-2025](https://mojang.atlassian.net/browse/MC-2025). As of right now, the Minecraft development team has not announced any information regarding fixes for the bug, so until they do there is no way to get around it.


##Staff

An up-to-date list of staff and their contact information can be found [here](https://docs.google.com/document/d/18zwbTgySU57wW1VAqdi5o34oLmarU77Me5smLYnUve8/edit).


##Terms

###Origin
The centre of the map. Located at **X: 0, Z: 0.**

This is a commonly frequented and very damaged area. Aggressors frequent Origin, so be careful.

###Null Road
The road originating at Origin and circumnavigating the world along the X and Z axes.

A specific subsection of the road is typically referred to in the form **-X Null Road** (i.e. signifying the Null Road going from **X: 0, Z: 0**, to **X: -5000, Z: 0**)

###Era
An Era is a creative way of referring to a specific segment of MinerApocalypse history.

Eras are separated by map resets. MinerApocalypse is currently on its tenth map, meaning it is in the **tenth era**.

###Role-play
Assuming a character in-game and using that character to advance a story or purpose in the game.

For example, a player may wish to roleplay as an adventurer/warrior by traveling across the land and killing those who wrong them.

###Combat Tagged
Whenever another player attacks the player, the player is considered to be "combat tagged" for **40 seconds**.

If a player logs out while combat tagged, then that player will leave behind an NPC representation of themselves where they logged off for a period of time.

If this NPC is killed by other players, then the player's items will drop and once the player logs in again, it will be as if they had died.

This is used so that players cannot escape from fights by logging off, causing an unfair game-play disturbance to the aggressor.


##Links

A list of useful links which are directly or tangentially related to the MinerApocalypse server.

###Server IP
[minerap.com](https://www.minerap.com/)

###MinerApocalypse Official Website
[minerap.com](https://www.minerap.com/)

###MinerApocalypse Subreddit
[reddit.com/r/minerapocalypse](http://www.reddit.com/r/minerapocalypse/)

###MinerAp Diplomacy XI
[reddit.com/r/minerapocalypse/comments/1oqhrk/minerap_diplomacy_xi/](http://www.reddit.com/r/minerapocalypse/comments/1oqhrk/minerap_diplomacy_xi/)

###MinerApocalypse Timeline
[docs.google.com/document/d/1xTxsTgZsKBdaKwRHZalgDe1It5VLD9QXx8y5untayFc](https://docs.google.com/document/d/1xTxsTgZsKBdaKwRHZalgDe1It5VLD9QXx8y5untayFc/)

###MinerApocalypse Plug.DJ Channel
[plug.dj/miner-apocalypse](http://plug.dj/miner-apocalypse/)
