Note
====

Create JSON and PNG
-------------------

    msdf-bmfont -f json -o kanjialive.png -t msdf --pot --square --smart-size -i kanjialive.txt kanjialive.ttf


Invert the PNG
--------------

Add

    negate: false

to the text component in Aframe.  Without this, the glyph renders as inverted.


Install into _assets_ folder
----------------------------

MUST rename font.json to font-msdf.json.

    cp font.png ../assets
    cp font.json ../assets/font-msdf.json

