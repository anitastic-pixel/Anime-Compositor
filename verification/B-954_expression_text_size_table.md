# B-954: Ctrl + wheel sizes the expression text (D-1074)

Written by `b954_expression_text_size` in `app/src/tests/w12x.rs`.

After Effects' Ctrl + wheel over an expression (4GeGLgK2BBk, about 3:40). `exprSizeStep` is the page's own code run under Node; the other rows read the page.

**11 of 11 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| no size kept yet, one notch up (Ctrl + wheel away from you) | 13 | 13 | pass |
| no size kept yet, one notch down | 11 | 11 | pass |
| five notches up from 12 | 17 | 17 | pass |
| forty notches up: no bigger than | 32 | 32 | pass |
| forty notches down: no smaller than | 8 | 8 | pass |
| no turn: the kept size | 15 | 15 | pass |
| a kept size that is not a number: the 12 every box starts at | 12 | 12 | pass |
| the expression box listens for Ctrl + wheel and only for it | true | true | pass |
| the size is kept with the window's preferences | true | true | pass |
| every expression box takes the size | true | true | pass |
| the kept size is used when the window opens | true | true | pass |
