cd ~/projects/rabin_karp/
args=`cat args.dat`
cargo build
time ./target/debug/rabin_karp text.txt ${args}
cd ~/projects/boyer_moore/
cargo build
time ./target/debug/boyer_moore text.txt ${args}
unset args
