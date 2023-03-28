# build the binary
cargo build
if [ $? -ne 0 ]; then
	echo "cargo build failed"
	exit 1
fi

# otherwise the tests will concurrently
# causing errors by writing data to 'tmp.s'
export RUST_TEST_THREADS=1

cargo test

# test failed
if [ $? -ne 0 ]; then
	# print the tmp.s for debug
	cat tmp.s
	echo "cargo test failed"
	exit 1
fi
