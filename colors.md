#  ANSI escape code

___________________________________________________________________________________________________________
|   n    | Name                               | Note
|--------|------------------------------------|-------------------------------------------------------------
|   0    | Reset or normal                    | All attributes become turned off
|   1    | Bold or increased intensity        | As with faint, the color change is a PC (SCO / CGA) invention.
|   2    | Faint, decreased intensity, or dim | May be implemented as a light font weight like bold.
|   3    | Italic                             | Not widely supported. Sometimes treated as inverse or blink.
|   4    | Underline                          | Style extensions exist for Kitty, VTE, mintty, iTerm2 and Konsole.
|   5    | Slow blink                         | Sets blinking to less than 150 times per minute
|   6    | Rapid blink                        | MS-DOS ANSI.SYS, 150+ per minute; not widely supported
|   7    | Reverse video or invert            | Swap foreground and background colors; inconsistent emulation[27]
|   8    | Conceal or hide                    | Not widely supported.
|   9    | Crossed-out, or strike             | Characters legible but marked as if for deletion. Not supported in Terminal.app.
|  10    | Primary (default) font             |
|  11–19 | Alternative font                   | Select alternative font n − 10
|  20    | Fraktur (Gothic)                   | Rarely supported
|  21    | Doubly underlined; or: not bold    | Double-underline per ECMA-48,[5]: 8.3.117  but instead disables bold intensity on several terminals, including in the Linux kernel's console before version 4.17.
|  22    | Normal intensity                   | Neither bold nor faint; color changes where intensity is implemented as such.
|  23    | Neither italic, nor blackletter    |
|  24    | Not underlined                     | Neither singly nor doubly underlined
|  25    | Not blinking                       | Turn blinking off
|  26    | Proportional spacing               | ITU T.61 and T.416, not known to be used on terminals
|  27    | Not reversed                       |
|  28    | Reveal                             | Not concealed
|  29    | Not crossed out                    |
|  30–37 | Set foreground color               |
|  38    | Set foreground color               | Next arguments are 5;n or 2;r;g;b
|  39    | Default foreground color           | Implementation defined (according to standard)
|  40–47 | Set background color               | 
|  48    | Set background color               | Next arguments are 5;n or 2;r;g;b
|  49    | Default background color           | Implementation defined (according to standard)
|  50    | Disable proportional spacing       | T.61 and T.416
___________________________________________________________________________________________________________
