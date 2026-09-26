# B-48: how much memory the viewer may use

Written by `cargo test --test b48_memory`. Machine memory: 66.1 GB. Card: NVIDIA GeForce RTX 4070 Ti SUPER (DiscreteGpu), driver NVIDIA 610.74, Vulkan, 16.8 GB of its own memory.

**10 of 10 checks pass.**

| Check | Expected | Actual | Result |
|---|---|---|---|
| Windows says how much memory the machine has | yes | yes | pass |
| the cache every earlier table measured is unchanged: D-40's 1 GiB, 448 MiB of it for effects | 576 MiB + 448 MiB | 576 MiB + 448 MiB | pass |
| Automatic gives the viewer a quarter of the machine's memory, never less than 1 GiB | 16.5 GB | 16.5 GB | pass |
| Custom may give it at most three quarters | 49.6 GB | 49.6 GB | pass |
| Automatic splits as the 1 GiB does: seven sixteenths for effects | 7/16 | 7/16 | pass |
| made 64 MiB after holding 2.1 GB, it lets go of what does not fit | at most 64 MiB, split 36 + 28 | at most 64 MiB, split 36 + 28 | pass |
| frames 0, 100 and 239 at Full, twice each from each cache, are the same bytes as the first time | identical | identical | pass |
| Automatic gives the card half its own memory, as before | 8.4 GB | 8.4 GB | pass |
| and it opens with that | 8.4 GB | 8.4 GB | pass |
| Custom may give the card at most 85% of its memory | 14.3 GB | 14.3 GB | pass |
