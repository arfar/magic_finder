#!/bin/sh

CARGO_LOCATION="$HOME/.cargo/bin"

SEARCH_STRING=$(rofi -l 0 -p "Input card name" -dmenu)
$CARGO_LOCATION/magic_finder_search_with_rofi_with_args.sh $SEARCH_STRING

