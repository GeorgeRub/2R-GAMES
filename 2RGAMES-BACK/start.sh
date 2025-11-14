#/bim/bash

list_of_services=("user_get_exercises")

list_of_folders=(sam-*)

for fol in "${list_of_folders[@]}"; do
    echo "$fol"
done


for ser in "${list_of_folders[@]}"; do

    echo "Preparing lambda service $ser"

    cd ./$ser
    echo "Clean cargo for $ser"
#    cargo clean
    rm target/lambda/$ser/bootstrap
    echo "Build release for $ser"
    cargo build --release --target x86_64-unknown-linux-musl
    mkdir -p target/lambda/$ser
    cp target/x86_64-unknown-linux-musl/release/$ser target/lambda/$ser/bootstrap

#    ./build-project.sh

    cd ..

    echo "Finish preparing lambda service $ser"

done

echo "Starting server"

sam local start-api --debug
