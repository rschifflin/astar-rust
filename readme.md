A simple A* implementation written in Rust.
This program takes a pathfile as input. It expects all pathfiles to conform to the following restriction:

Pathfiles consist of `height` lines each of `width` characters. Each line must contain characters from the following character set:
- s
- f
- .
- |

Only one `s` character is allowed within the pathfile. It represents the start position of the path.
Only one `f` character is allowed within the pathfile. It represents the finish position of the path.
All `.` characters represent empty space, which can be pathed through.
All `|` characters represent occupied space, which cannot be pathed through.

This program will output the a solved pathfile, with the path from start to finish marked with `@` characters.

Ex:

input pathfile:

    s...
    ||.|
    g...
    ....

output pathfile:

    s@@.
    ||@|
    g@@.
    ....
