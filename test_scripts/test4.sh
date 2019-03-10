
target/release/li3-2019 | \
while read -r line
do
    if grep "$line" db/Vendas_1M._Valid.txt >/dev/null
    then
        echo product "$line" was sold
    fi
done
