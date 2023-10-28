# RPG: Random Password Generator

Generate a random password and calculate the password's entropy.

```bash
Usage: rpg [OPTIONS] [LENGTH]

Arguments:
[LENGTH] Password length [default: 16]

Options:
-p, --password-type <TYPE> Password type [default: random] [possible values: random, pin, memorable]
-n, --numbers Include numbers
-s, --symbols Include special symbols
-c, --caps Include capitalized letters
-h, --help Print help
-V, --version Print version
```

## Random

Generate a 10 character random password

```bash
$ rpg 10
rbfflarczr
47.00439718141092
```

Generate a 10 character random password with numbers and special
characters

```bash
$ rpg 10 -ns
/0shf*;yi|
58.57980995127572
```

Generate an 8 digit pin

```bash
rpg 8 -p pin
50803803
26.575424759098897
```

Generate a 5 word memorable pass phrase

```bash
rpg 5 -p memorable
hubcap-client-pushy-reawake-emphases
171.17595007788486
```

# Password Score

The password strength is estimated with a score of 0 to 100, using the
following rules.

_Rules credit_: https://www.uic.edu/apps/strong-password/

**Bonuses**

| Rule                      | Type      | Rate          |
| ------------------------- | --------- | ------------- |
| Length                    | Flat      | +(n\*4)       |
| Uppercase letters         | Cond/Incr | +((len-n)\*2) |
| Lowercase letters         | Cond/Incr | +((len-n)\*2) |
| Numbers                   | Cond      | +(n\*4)       |
| Symbols                   | Flat      | +(n\*6)       |
| Middle numbers or symbols | Flat      | +(n\*2)       |
| Requirements              | Flat      | +(n\*2)       |

**Penalties**

| Rule                                 | Type | Rate    |
| ------------------------------------ | ---- | ------- |
| Letters only                         | Flat | -n      |
| Numbers only                         | Flat | -n      |
| Repeat Characters (case insensitive) | Comp | -       |
| Consecutive uppercase letters        | Flat | -(n\*2) |
| Consecutive lowercase letters        | Flat | -(n\*2) |
| Consecutive numbers                  | Flat | -(n\*2) |
| Sequential letters (3+)              | Flat | -(n\*3) |
| Sequential numbers (3+)              | Flat | -(n\*3) |
| Sequential symbols (3+)              | Flat | -(n\*3) |

Flat
: Rates that add/remove in non-changing increments.

Incr
: Rates that add/remove in adjusting increments.

Cond
: Rates that add/remove depending on additional factors.

Comp
: Rates that are too complex to summarize. See source code for details.

n
: Refers to the total number of occurrences.

len
: Refers to the total password length.
