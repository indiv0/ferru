Title: Arch Linux: Take Two
Date: 2014-06-03
Tags: linux, wm, dwm, bspwm, arch
Category: blog
Slug: arch-linux-take-two
Author: Nikita Pekin
Summary: My re-installation and re-configuration of Arch Linux.
Status: draft

First, I installed Arch Linux.
I won't bother writing down a instructions here, as the [beginner's guide](https://wiki.archlinux.org/index.php/beginners%27_guide) and [installation guide](https://wiki.archlinux.org/index.php/Installation_guide) are excellent resources which do a much better job of explaining the installation procedure than I could.

The only pre-requisites are that the system has an internet connection, you've updated pacman's repos (`pacman -Syy`), and that you've enabled the `[multilib]` repo in `/etc/pacman.conf` (if you are using a 64-bit system).

Download and run the setup script:
    curl -O https://raw.githubusercontent.com/Indiv0/pupfiles/master/setup.sh
    chmod +x setup.sh
    ./setup.sh

The script will download and install `yaourt` and `ansible`, then call `ansible` to provision the rest of the system:



After seeing [this](http://www.reddit.com/r/unixporn/comments/204zd1/archbspwm_fresh_and_simple/) beautiful bspwm setup, I decided to give bspwm a try.

I used windelicato's [bspwm for dummies](https://github.com/windelicato/dotfiles/wiki/bspwm-for-dummies) guide as a tutorial.

First, I installed bspwm itself from the AUR:

    yaourt -S bspwm

I then got the default configuration files for bspwm and sxhkd:

    mkdir -p ~/.config/bspwm ~/.config/sxhkd
    curl https://raw.github.com/baskerville/bspwm/master/examples/bspwmrc > ~/.config/bspwm/bspwmrc
    curl https://raw.github.com/baskerville/bspwm/master/examples/sxhkdrc > ~/.config/sxhkd/sxhkdrc

As suggested in windelicato's guide, I changed every reference of `super` to `alt` in `~/.config/sxhkd/sxhkdrc`, because I also prefer using the alt key over super.

On line 84, I changed the terminal client to `urxvtc`.

* 
* Firefox Sync recovery key


* Followed [Beginners' Guide](https://wiki.archlinux.org/index.php/beginners'_guide).
* Boot into Windows 8.1 setup disk
* Follow setup instructions, leaving space on SSD for Arch Linux
* Enter Windows 8.1, [set windows time to UTC](https://wiki.archlinux.org/index.php/Time#UTC_in_Windows).
* Boot into Arch Linux x86-64 ISO.
* Set locale:

    # nano /etc/locale.gen
    en_CA.UTF-8 UTF-8

    locale-gen
    export LANG=en_CA.UTF-8

* Ensure I have internet access:

    ping -c 3 google.com

* Stop `dhcpcd` service:

    systemctl stop dhcpcd

* Get Ethernet interface name:

    ip link

    * Mine was enp3s0

* Setup static IP

    ip link set enp3s0 up
    ip addr add 192.168.1.15/24 dev enp3s0
    ip route add default via 192.168.1.1

    * My static IP is `192.168.1.1` and my gateway IP is `192.168.1.1`

* Check connection again:

    ping -c 3 google.com

* I'm using MBR/GRUB (not UEFI or GPT) with only one partition (for `/`), so my partition setup is simple:

    cfdisk /dev/sda
    
    * Delete the partition where you will be installing Arch, if it already occupied (ignore this if you are doing a clean install).
    * Choose `[ New ]` on the `Free Space` area.
    * Choose `[ Primary ]` partition.
    * Don't modify the size parameter.
    * Choose `[ Write ]` to write the partition scheme to the disk.
    * Type `yes` at the prompt.
    * Take note of the partition name (e.g. `/dev/sda3`).
    * Choose `[ Quit ]`

* Create the filesystem on the newly created partition.

    mkfs.ext4 /dev/sda3

* Mount the partition:

    mount /dev/sda3 /mnt
    
* Select fastest Canadian mirrors using `reflector`:

    pacman -Syy
    pacman -S reflector
    reflector --verbose --country 'Canada' -p http --sort rate --save /etc/pacman.d/mirrorlist

* Install the base system:

    pacstrap -i /mnt base base-devel

* Generate an fstab and verify that it's correct:

    genfstab -U -p /mnt >> /mnt/etc/fstab
    nano /mnt/etc/fstab

* Chroot into the base system:

    arch-chroot /mnt /bin/bash

* Configure locale:

    # nano /etc/locale.gen
    en_CA.UTF-8 UTF-8
    
    locale-gen
    echo LANG=en_CA.UTF-8 > /etc/locale.conf
    export LANG=en_CA.UTF-8

* Configure time zone:

    ln -s /usr/share/zoneinfo/Canada/Eastern /etc/localtime

* Set the hardware clock to UTC (this will mess with the Windows clock unless you set it to UTC as mentioned above):

    hwclock --systohc --utc

* Set the hostname to your preference (I'm following the periodic table) and make sure to modify the hosts definition for `127.0.0.1` to include it:

    echo helium >> /etc/hostname

    # nano /etc/hosts
    127.0.0.1	localhost.localdomain	localhost	helium

* Configure netctl for static IP using an example netctl profile (lets call my network `pffwf`):

    cd /etc/netctl
    cp examples/ethernet-static pffwf
    nano pffwf
    
    netctl enable pffwf
    
    * Use the same internet settings as before (IP, gateway, subnet, interface, etc.)
    
* Create an initial ramdisk environment

    mkinitcpio -p linux
    
* Set the root password:

    passwd
    
* Install and configure `grub` (`os-prober` needed for `grub` to detect Windows 8.1):

    pacman -S grub os-prober
    grub-install --target=i386-pc --recheck /dev/sda
    grub-mkconfig -o /boot/grub/grub.cfg
    
* I like to change the Windows 8.1 manuentry title in grub to `Windows 8.1` instead of `Windows 8 (loader) (on /dev/sda1)`, so I modify `/boot/grub/grub.cfg`:

    nano /boot/grub/grub.cfg

* Leave the chroot, unmount the partitions, boot into Arch:

    exit
    umount -R /mnt
    reboot

## Arch Linux ##

* Login as `root` with the password you set before with `passwd`.
* To avoid continually using `root` (a bad habit), we'll get `sudo` and create a new user (replace `USER` with your username):

    pacman -S sudo
    useradd -m -G wheel -s /bin/bash USER
    passwd USER
    
    # nano /etc/sudoers
    USER ALL=(ALL) ALL
    
    * Avoid giving users `NOPASSWD` sudo access.
      This reduces your system's security.
      
* Login as the new user:

    logout

* Update the pacman config to include `multilib`:

    sudo nano /etc/pacman.conf

* Install git, clone the setup script, and run it:

    sudo pacman -S git
    
    git clone https://gist.github.com/74464459bf73120a66a0.git setup
    setup/setup.sh
    
* Setup git and ssh (make sure to put a passphrase on your newly generated SSH keys):

    ssh-keygen -t rsa -C "indiv0@helium"
    git config --global user.email "not.actually.an@email.com"
    git config --global user.name "Nikita Pekin"
    
* Add your `~/.ssh/id_rsa.pub` public key to your Github keys.
    * Probably want to do this with:
        
        # vim .xinitrc
        exec bspwm &
        firefox

        chmod +x .xinitrc
        
        startx

* Configure vcsh:

    vcsh dotfiles remote add origin git@github.com:Indiv0/dotfiles.git
    rm .xinitrc .gitconfig .packages-aur
    vcsh dotfiles pull
    
* Remove extra packages by repeatedly running:

    yaourt -R $(yaourt -Qdtq)
    
TODO:

    * Remove:
        * .bash_logout
        * .bash_profile
        * .bashrc
    * Leave:
        * .local/
