#!/bin/sh

CARGO_SCRIPT_LOCATION="$HOME/.cargo/bin"

SCRYFALL_BULK=$(rofi -modi filebrowser -show filebrowser -filebrowser-command printf)
echo "$SCRYFALL_BULK"

$CARGO_SCRIPT_LOCATION/magic_finder --update "$SCRYFALL_BULK"

# TODO - check return value
rofi -e "Your database should be updated now"
