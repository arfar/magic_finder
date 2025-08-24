#!/bin/sh

CARGO_SCRIPT_LOCATION="$HOME/.cargo/bin"

# Note to self in this... The whitespace seemed to fuck the ifs up. Not sure why.
# This is why it's all flat and ugly

CARDS=$($CARGO_SCRIPT_LOCATION/magic_finder $@)
RETURN=$?

#######################
## Exact card found - just print the card
#######################
if [ $RETURN -eq 200 ]; then

rofi -e "$CARDS"

fi


#######################
## Cards to select from
#######################
if [ $RETURN -eq 106 ]; then

SELECTION=$(rofi -dmenu -i << EOF
$CARDS
EOF
)

CARD_OUTPUT=$($CARGO_SCRIPT_LOCATION/magic_finder --exact $SELECTION)

if [ -z "$CARD_OUTPUT" ]; then
   # No card selected - most likely an early exit
   exit 1
fi

# If you double check the a rofi selection it seems to prevent the error window from popping up
#  I think this is because it registers the second click as a click outside the window which exits
#  the rofi -e message
sleep 0.05

rofi -e "$CARD_OUTPUT"
fi

##########################
## Not even one card that matched - try a close string
##########################
if [ $RETURN -eq 105 ]; then

# TODO do something different with no matching string at all - perhaps even a different ExitCode?

SELECTION=$(rofi -dmenu -p "Did you mean?" -i << EOF
$CARDS
EOF
)

if [ -z "$SELECTION" ]; then
   # Nothing word - most likely an early exit
   exit 1
fi

CARDS=$($CARGO_SCRIPT_LOCATION/magic_finder $SELECTION)
RETURN=$?

## In this case - the selected closest word only has one option. See "Skuller" > "Sculler" for an example
if [ $RETURN -eq 200 ]; then
    sleep 0.05
    # Not actually $CARDS in this case - it's just 1 card.
    rofi -e "$CARDS"
    exit
fi

SELECTION=$(rofi -dmenu -i << EOF
$CARDS
EOF
	 )

if [ -z "$SELECTION" ]; then
   # No card selected - most likely an early exit
   exit 1
fi

CARD_OUTPUT=$($CARGO_SCRIPT_LOCATION/magic_finder --exact $SELECTION)

sleep 0.05

rofi -e "$CARD_OUTPUT"

fi

###############################
## No seach string input at all
###############################
if [ $RETURN -eq 101 ]; then

rofi -e "No search string found"

fi
