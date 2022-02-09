CACHE_DIR=rust-cache
if [ ! -f $CACHE_DIR/cache-date ]; then
    echo "No cache found"
    exit 0
fi
tar -zxvf $CACHE_DIR/target.tar.gz -C .
tar -zxvf $CACHE_DIR/cargo-home.tar.gz -C $CARGO_HOME/..