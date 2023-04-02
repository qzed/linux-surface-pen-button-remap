#!/bin/bash

echo "Available keys: 
KEY_ESC
KEY_1
KEY_2
KEY_3
KEY_4
KEY_5
KEY_6
KEY_7
KEY_8
KEY_9
KEY_0
KEY_MINUS
KEY_EQUAL
KEY_BACKSPACE
KEY_TAB
KEY_Q
KEY_W
KEY_E
KEY_R
KEY_T
KEY_Y
KEY_U
KEY_I
KEY_O
KEY_P
KEY_LEFTBRACE
KEY_RIGHTBRACE
KEY_ENTER
KEY_LEFTCTRL
KEY_A
KEY_S
KEY_D
KEY_F
KEY_G
KEY_H
KEY_J
KEY_K
KEY_L
KEY_SEMICOLON
KEY_APOSTROPHE
KEY_GRAVE
KEY_LEFTSHIFT
KEY_BACKSLASH
KEY_Z
KEY_X
KEY_C
KEY_V
KEY_B
KEY_N
KEY_M
KEY_COMMA
KEY_DOT
KEY_SLASH
KEY_RIGHTSHIFT
KEY_KPASTERISK
KEY_LEFTALT
KEY_SPACE
KEY_CAPSLOCK
KEY_F1
KEY_F2
KEY_F3
KEY_F4
KEY_F5
KEY_F6
KEY_F7
KEY_F8
KEY_F9
KEY_F10
KEY_NUMLOCK
KEY_SCROLLLOCK
KEY_KP7
KEY_KP8
KEY_KP9
KEY_KPMINUS
KEY_KP4
KEY_KP5
KEY_KP6
KEY_KPPLUS
KEY_KP1
KEY_KP2
KEY_KP3
KEY_KP0
KEY_KPDOT
KEY_102ND
KEY_F11
KEY_F12
KEY_KPENTER
KEY_RIGHTCTRL
KEY_KPSLASH
KEY_SYSRQ
KEY_RIGHTALT
KEY_HOME
KEY_UP
KEY_PAGEUP
KEY_LEFT
KEY_RIGHT
KEY_END
KEY_DOWN
KEY_PAGEDOWN
KEY_INSERT
KEY_DELETE
KEY_POWER
KEY_KPEQUAL
KEY_PAUSE
KEY_LEFTMETA
KEY_RIGHTMETA
KEY_COMPOSE
KEY_F13
KEY_F14
KEY_F15
KEY_F16
KEY_F17
KEY_F18
KEY_F19
KEY_F20"
echo "Configure action: Single Click"
while :; do

  read -r -p 'First key: ' fkey
  read -r -p 'Second key (n for none): ' skey
  read -r -p 'Third key (n for none): ' tkey
  if [ $skey == 'n' ]; then
  	single=$(echo "            single: vec![EV_KEY::$fkey],")
  elif [ $tkey == 'n' ]; then
  	single=$(echo "            single: vec![EV_KEY::$fkey, EV_KEY::$skey],")
  else
  	single=$(echo "            single: vec![EV_KEY::$fkey, EV_KEY::$skey, EV_KEY::$tkey],")
  fi

  echo "Configure action: Double Click"
  while :; do

    read -r -p 'First key: ' fkey
    read -r -p 'Second key (n for none): ' skey
    read -r -p 'Third key (n for none): ' tkey
    if [ $skey == 'n' ]; then
    	  double=$(echo "            double: vec![EV_KEY::$fkey],")
    elif [ $tkey == 'n' ]; then
  	  double=$(echo "            double: vec![EV_KEY::$fkey, EV_KEY::$skey],")
    else
  	  double=$(echo "            double: vec![EV_KEY::$fkey, EV_KEY::$skey, EV_KEY::$tkey],")
    fi
    
    echo "Configure action: Hold"
  	while :; do

    	read -r -p 'First key: ' fkey
    	read -r -p 'Second key (n for none): ' skey
    	read -r -p 'Third key (n for none): ' tkey
    	if [ $skey == 'n' ]; then
    	  	hold=$(echo "            hold: vec![EV_KEY::$fkey],")
    	elif [ $tkey == 'n' ]; then
  	  	hold=$(echo "            hold: vec![EV_KEY::$fkey, EV_KEY::$skey],")
    	else
  	  	hold=$(echo "            hold: vec![EV_KEY::$fkey, EV_KEY::$skey, EV_KEY::$tkey],")
    	fi
    	echo "Single click: $single"
    	echo "Double click: $double"
    	echo "Hold: $hold"
    	
    	sed -i "59s/.*/$single/" src/main.rs
    	sed -i "60s/.*/$double/" src/main.rs
    	sed -i "61s/.*/$hold/" src/main.rs
    	
    	echo "The actions are configured! Use './install.sh' to install this program and your configuration!"
    	break

  	done
  	break

  done
  break

done
