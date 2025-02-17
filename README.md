# Garfield Esperanto

1000 *Garfield* comics translated to Esperanto.

-   [See website live now!](https://dxrcy.dev/garfeo) (in Esperanto)

Made with [Ibex](https://github.com/dxrcy/ibex)

# File Structure

```
/assets/posts
    /<index>        Index in order of creation date, padded to 4 digits
        /esperanto.png
        /english.png
        /esperanto.svg  Unexported SVG of *text replacement* (only for recent entries)
        /title      Post title/caption
                        eg. Garfildo ŝatas lazanjon
        /date       Date of original comic YYYY-MM-DD
                        eg. 2012-10-01
        /transcipt  Transcription of speech in comic
        /props      If any special properties (optional)
                        possible values (separated by linebreak):
                            good, nogarfield, earsback
        /special    If comic is for a special occasion (Eg. Christmas)
        /notes      If anything is worth noting about the comic
```

