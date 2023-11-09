# Garfield Esperanto

500+ *Garfield* comics translated to Esperanto.

-   [See website live now!](https://darccyy.github.io/garfeo) (in Esperanto)

Made with [Ibex](https://github.com/darccyy/ibex)

# File Structure

```
/static/posts
    /<index>        Index in order of creation date, padded to 4 digits
        /esperanto.png
        /english.png
        /esperanto.svg  Unexported SVG of *text replacement* (only for recent entries)
        /title      Post title/caption
                        eg. Garfildo ŝatas lazanjon
        /date       Date of original comic YYYY-MM-DD
                        eg. 2012-10-01
        /errata     If any errors exist in the translation (optional)
                        eg. lasagno >> lazanjo
                            garfish >> garfield
        /props      If any special properties (optional)
                        possible values (separated by linebreak):
                            good, nogarfield, notext, earsback
```

