CACHE_DIR=rust-cache
if [ -f $CACHE_DIR/cache-date ]; then
    cache_date=$(cat $CACHE_DIR/cache-date)
    now=$(date +%s)
    diff=$(($now-$cache_date))
    maxage=604800 # Seven days, as per Bitbucket Pipelines caching policy
    if [ "$diff" -lt "$maxage" ]; then
        echo "Existing fresh cache found, not building a new one"
        exit 0
    fi
fi

mkdir -p $CACHE_DIR
date +%s > $CACHE_DIR/cache-date
tar -zcvf $CACHE_DIR/target.tar.gz -C . target/
tar -zcvf $CACHE_DIR/cargo-home.tar.gz -C $CARGO_HOME/.. cargo/