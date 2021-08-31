for d in */ ; do
    printf "Formatting $d...\n"
    cd $d
    cargo fmt
    cd ..
done
read -n 1 -s -r -p "Press any key to exit "
printf "\n"
