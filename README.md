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
