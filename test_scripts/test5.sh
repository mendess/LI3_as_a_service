#!/bin/bash

while read -r line
do
    if [ "$(grep "$line" db/Vendas_1M._Valid.txt | cut -d' ' -f7 | sort | uniq | wc -l)" != "3" ]
    then
        echo "$line"
    fi
done < db/Clientes_Valid.txt
