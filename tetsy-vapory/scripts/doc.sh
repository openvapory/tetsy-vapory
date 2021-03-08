#!/usr/bin/env sh
# generate documentation only for partiy and vapcore libraries

cargo doc --no-deps --verbose --all --exclude tetsy-ipfs-api &&
	echo '<meta http-equiv=refresh content=0;url=vapcore/index.html>' > target/doc/index.html
