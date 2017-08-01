cat /dev/urandom | tr -dc 'a-zA-Z' | fold -w 10 | head -100 | sort | uniq | tr '\n' ' ' > args.dat
