# Magic Finder

A quick way to search up Magic the Gathering (TM) cards for Linux (maybe MacOS? Don't have one, so haven't tried).

Scroll down to see how this works (with the optional `rofi` integration`). Currently, no images are displayed - so don't go into this expecting that.

## The Components

This repo has 2 main parts to it:

 * The `magic_finder` rust code, which does the "heavy lifting" of updating a database, searching through it for cards, and finding close names (kind of)
 * The supporting scripts which use [`rofi`](https://github.com/davatorium/rofi)
 
`magic_finder` can be used without the rofi parts if you wanted a command for showing basic mtg card info from the CLI.

The `rofi` part is so that I can quickly and easily get the card info I want. Basically just adds a very simple and easy GUI to the `magic_finder` part. I've written 2 wrapper scripts to enable this.
 
## Requirements

### Magic Finder
`magic_finder` is written in rust. Check the Cargo.toml file for more specific requirements. I was compiling with rustc version 1.88, but I would not at all be surprised if it worked on rust from year(s) ago.
 
### `rofi`
The rofi scripts require a version that can set the `command` setting on the `filebrowser` option. From what I can tell, this was introduced in 1.7.6, so you'll need a version at or above this.

Very annoyingly, the Ubuntu repos do *not* have this version. They have an earlier version. So, to use this, you'll need to get a more recent version yourself. I compiled and installed myself. See the end of this README for how I did that.

I think `rofi` only works with Linux, maybe on MacOS, almost certainly not on Windows. For quick desktop-based search, you'll need to run Linux (or maybe MacOS). I haven't tested MacOS because I don't have access to a machine that runs it.

## Example Usage (with `rofi`)

Let's try find Black Lotus, but we did a typo. Run the `magic_finder_search_with_rofi.sh` script and input your typo.

![search with typo](images/first.png)

Because no cards are found with the term "blakc", it'll suggest what you might've meant.

![search result of the typo](images/misspelled.png)

Now it'll re-do the search with the correctly spelled word (unfortunate for us "black" is a common word in magic, so we're presented with more options)

![black search output](images/black_output.png)

Luckily you can filter further by typing in some more words like "Lotus").

![with "lotus" now to filter further](images/search_lotus.png)

And finally hit enter or double click on the card you want to get the output you need.

![black lotus output](images/black_lotus_card.png)

### Example Usage Without Rofi

I have not coded any sort of similar "interactive" mode for the `magic_finder` tool itself. As such, you'll need to do the same steps yourself, but manually and without interactive search.

```
$ magic_finder blakc
black
blast
... <SNIP> ...
```

```
$ magic_finder black
Argivian Blacksmith
Ballad of the Black Flag
... <SNIP> ...
```

```
$ magic_finder --exact Black Lotus
Black Lotus	{0}
Artifact
{T}, Sacrifice this artifact: Add three mana of any one color.
Scryfall URI: https://scryfall.com/card/vma/4/black-lotus?utm_source=api
```

Without exact, this tool will imitate Scryfall search and search for each individual word in the card.

```
$ magic_finder Black Lotus
Black Lotus
Black Lotus Lounge
Blacker Lotus
```

## Installation, First Usage, and Updating

### Installation
I am sorry in advance, this is a bit of a pain becuase of my lack of knowledge on how to properly "package" scripts alongside a rust binary.

You will need `rust`, so please install that before you start. Follow the insrtructions [here](https://www.rust-lang.org/tools/install).

#### Install the `magic_finder` binary

Clone this repo and move into the directory using something like

```
git clone https://github.com/arfar/magic_finder
cd magic_finder
```

Then build and install the package.

```
cargo install --path .
```

Hopefully `cargo` has installed this somewhere that's already in `$PATH` and you should now already be able to use the binary. Run `magic_finder --help` to test.

##### Install the `rofi` scripts (optional)

Install `rofi` with a version > 1.76 first. As of writing, you can't simply do `sudo apt install rofi` for Ubuntu - see the section below about how to compile and install `rofi` locally. 

This is the slightly harder part. You need to put these scripts somewhere in `$PATH` (I put them in the same place that `cargo` installed the binary by default).

For me, an easy and logical-ish place was the same place that the `magic_finder` binary was installed. Run the following to check:

```
which magic_finder # gives /home/<USER>/.cargo/bin/magic_finder for me
```

So given the above output, `$HOME/.cargo/bin` is the folder I want to use for installing these scripts. I know it's not a cargo binary - but this should be a location that is in `$PATH` already, and it's "local" to the binary itself too.

**THIS IS THE MOST PAINFUL PART** - you need to manually edit the `CARGO_SCRIPT_LOCATION` variable in each of the files in the `scripts/` folder. The deafult is *probably* correct with `$HOME/.cargo/bin`, but if that's not in your `$PATH`, you will need to change that. So change all of the files if needed.

Then "install" the scripts by copying them all from the `magic_finder/scripts` folder and put them in that same `CARGO_SCRIPT_LOCATION` folder. It's probably the following command if you're still in the `magic_finder` folder. That's what I use with default rust/cargo settings/setup.

```
cp scripts/* $HOME/.cargo/bin/
```

With that all sorted, you should try test it out! Presuming you've already `magic_finder` and it's working, try running `magic_finder_update_with_rofi.sh`. This should give you a file browser. Exit using <Ctrl+g> or <Esc>. If so, then scripts are working!


##### All In One
With rust basic defaults, the following might just work for you... (it does for me)
```
git clone https://github.com/arfar/magic_finder
cd magic_finder
cargo install --path .
cp scripts/* $HOME/.cargo/bin
```
Then test by opening a terminal window then running
```
magic_finder --help
magic_finder_search_with_rofi.sh
```

#### Install Shortcut
I use Ubuntu and the real helpfulness of this script is the ability to call this from a quick keyboard shortcut. The `magic_finder_search_with_rofi.sh` script is for use with a keyboard shortcut. Do whatever works with your OS, but for me on Ubuntu, I went to the `Settings` application. Then `Keyboard` > `Keyboard Shortcuts` > `Custom Shortcuts` > `+`. When adding the shortcut, provide it the location of the script (probably `$HOME/.cargo/bin/magic_finder_search_with_rofi.sh`) and the key binding (`SUPER + s` for me).

With this set up, pressing `SUPER + s` will provide a basic `dmenu`-esque `rofi` menu where you type the card you're looking for - and you should just be off.

### Once You Install and Updating the Database
Go to the [Scryfall Bulk Download](https://scryfall.com/docs/api/bulk-data) page and download the Oracle Cards file. Should be 150MB-ish.

If you're using `rofi` then use `magic_finder_update_with_rofi.sh` to find the file, or use `magic_finder --update <LOCATION_OF_FILE>` where `<LOCATION_OF_FILE>` is where you downloaded the file to. From there, it should Just Work (TM). If not, try updating this repo. If it still doesn't work, log a ticket. It's probably going to something with Scryfall updating their schema that I haven't accounted for. Alternatively, run `COMMAND --update <path to file>` where `<path to file>` is the full path to where you downloaded the file.

NOTE: Updating *will* delete the previous db - that shouldn't be a problem though, because you shouldn't use that unless you really know what you're doing.

## Uninstall

Firstly, find where the database is installed using the following.

```
magic_finder --database-folder
```

Delete the whole folder that is output from that command. It's probably somewhere like `$HOME/.local/share/magic_finder/`.

The binary and scripts should be in the folder when you run `which magic_finder`. Delete all files that start with `magic_finder`.

Optioanally, from within the `magic_finder` folder, run `cargo uninstall`. This will remove the binary and it should tell you where that was. Hopefully the scripts are in the same place, so go to that folder and delete the remainder of the files that start with `magic_finder`.
 
## Why this exists

I like watch Magic the Gathering (TM) videos, expecially while coding, working, writing, whatever. Often, I don't know what card they're talking about. They'll often say the card name (sometimes a nickname - this tool doesn't help with that), and show it on the screen briefly (or in a tiny/obscured view), and I'll miss what it actually does. When this happens, I need to open a tab on my browser, go to [Scryfall](scryfall.com), type in the name, (sometimes) click the specific card, and the view it. This takes 2-3 page loads, changing my active window and is just a bit of a pain.

This tool in coordination with `rofi` enables me to hit `META+S`, type in the card name, navigate to the card (if needed) with my keyboard, and display the card. No browser, no HTTP, lower context switch, instant card displayed right there, and goes away when I press anything else.

The idea is it's just easier and quicker than my normal process.

## How I Installed `rofi`
If you're smarter than me, just follow the official docs: https://github.com/davatorium/rofi/blob/next/INSTALL.md and don't bother reading this.

I am entirely unfamiliar with `meson` and `ninja` (I'm more a `Makefile` kinda guy - haven't done any `C` properly in >10 years), so here's what I did. This is for Ubutntu - you will need to do something different for download the dependencies (you can see them in the INSTALL.md file referenced above).

Of note below, I'm installing this into my `$HOME/bin` directory. Change that part if you want to install somewhere else. Make sure to install it somewhere in your `$PATH` though!

Clone the repo and move into it

```
git clone --recursive https://github.com/davatorium/rofi
cd rofi
```

Install the deps

```
sudo apt build-dep rofi
sudo apt install meson
sudo apt install libxcb-keysyms1-dev libxcb-keysyms1 # I suspect only one of these is needed - not sure which
```

Setup and build (not sure why I put the prefix in here... you'll see below I still copy+pasted the bin)
```
meson setup build --prefix $HOME/bin -Dwayland=disabled -Dxcb=enabled
ninja -C build -v
```

Fingers crossed that all compiled and stuff... then copy the bin

```
cp build/rofi ~/bin
```

## Potential Features
 * Display the actual card image (probably won't do this)
 * Add some classic nicknames (might be difficult to find them all). examples include:
   - Bob - Dark Confidant
   - AK - Accumulated Knowledge
   - find more here: https://mtg.wiki/page/List_of_Magic_slang/Card_nicknames

## TODO / FIXME / BUGS / Code improvements
 * Do some kind of "Display All" kind of thing
   For example, searching "Tezzeret" gives a bunch of cards and I'm not sure which one I want (other than probably the type - but there's still a bunch of Planeswalkers)
 * Figure out (if possible) to make the Scryfall URI (L?) clickable in `rofi`.
 * Optionally, put the Scryfall URI into the clipboard.
 * Double optionally, provide some kind of `xdg-open <SCRYFALL_LINK>` sort of thing.
 * Test the DbErrors stuff.
 * For misspelled cards, if only 1 hit that makes sense, could just work and/or provide the specific card alongside the other spellings
 * Some kind of auto-magic direct link between the `ExitCode`s set out in `main.rs` and the `rofi` scripts. Currently I need to manually make sure they're the same between the `rust` code and the `sh` code. I think `build.rs` could do something like this.
   I'm guessing could involve cargo build.rs (or just a find+replace?)
 * Add more tests and improve the ones in `deser.rs`
 * Reduce `deser.rs` to only relevant key:value pairs or seperate into different repo/module entirely. It could maybe be useful for others.
   - Optionally/Alternatively delete the stuff in `deser.rs` that I don't use.
 * Fix all of the unchecked Results and deal with them (or just panic)

## Thanks

This project really is just 95% based on Scryfall. They're amazing. I don't know how or why they exist, but I think they're basically the best Magic the Gathering (TM) resource online.

Of course `rofi` for providing the quick, simple, low-weight, and well documented tool. Particularly the quickness and low-weight which made it really possible. The alternative was opening terminal windows, or using TKinter or something... surely not worth it.

Thanks to all the amazing `rust` packages I use.
