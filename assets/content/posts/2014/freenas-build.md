title: My FreeNAS Build
date: 2014-08-21
tags: bsd, freebsd, nas, freenas, hardware
category: blog
slug: freenas-build
author: Nikita Pekin
summary: Constructing and setting up a FreeNAS system.
status: draft

This is my post documenting my FreeNAS build.

# Hardware

* TBD

# OS

## Memtest86+

* Test RAM with Memtest86+ following [these](http://forum.canardpc.com/threads/28875-Linux-HOWTO-Boot-Memtest-on-USB-Drive) instructions.

## FreeNAS
### Installation
* [Download](http://www.freenas.org/download-freenas-release.html) the latest FreeNAS release USB image.
* Extract the downloaded file with `xzcat FreeNAS-9.2.1.7-RELEASE-x64.img.xz | sudo dd of=/dev/sdd bs=64k`
* Plug the USB into the NAS and boot with it.
* Let FreeNAS set itself up. The first boot takes a LONG time.
* Use the IP address provided at the end of the installation to connect to the FreeNAS web interface.

### Users
* Create a user via "Account > User > Add User", with the primary group as `wheel`.

### Volumes
* Create a ZFS volume.

### CIFS
* Under "Services", enable `CIFS`.
* On the UNIX client, edit the `/etc/fstab` to contain:

    //freenas.local/media   /media      cifs    users,credentials=/etc/smbcredentials,workgroup=WORKGROUP,ip=192.168.1.134 0 0

* Create a `~/.smbcredentials` file with the contents:

    username=your_username
    password=your_password

* Mount the CIFS share with a command like:

    mount.cifs //freenas.local/media /media

### Plex

* Click "Plugins" at the top of the menu.
* Click "plexmediaserver"
* Click "Install"

### TODO

* Set up static DNS
* Set up plexmediaserver
* Set up UPnP
* Set up UPS
* Set up regular ZFS scrubs
* Set up S.M.A.R.T.
