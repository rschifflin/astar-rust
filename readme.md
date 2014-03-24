A simple A* implementation written in Rust.
This program takes a pathfile as input. It expects all pathfiles to conform to the following restriction:

The first line of the path file is of the form `<number>x<number>`, denoting the width x height of the rest of the file.
This line is followed by `height` subsequent lines. Each line must contain `width` characters from the following character set:
-- s
-- f
-- .
-- x

Only one `s` character is allowed within the pathfile. It represents the start position of the path.
Only one `f` character is allowed within the pathfile. It represents the finish position of the path.
All `.` characters represent empty space, which can be pathed through.
All `x` characters represent occupied space, which cannot be pathed through.

This program will output the a solved pathfile, with the path from start to finish marked with `*` characters.

Ex:

input pathfile:
4x4
s...
xx.x
g...
....

output pathfile:
4x4
s**.
xx*x
g**.
....
