#! /usr/local/bin/gforth

: HELLO ." hello world " CR ;

: HELLO200 200 0 DO HELLO LOOP ;

HELLO200 bye
