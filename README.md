ID=dev-1651852369664-24990959096739
echo $ID

Inicializar contrato:

near call $ID init_contract '{"owner_id":"'$ID'"}' --accountId $ID

Obtener producto

    near view $ID get_products '{"address":"0x1"}'
    near view $ID get_products '{"address":"0x2"}'
    near view $ID get_products '{"address":"0x3"}'

Guardar producto

    near call $ID set_products '{"address":"0x1", "name":"zapatos", "price": 250, "stock":5}' --accountId yairnava.testnet
    near call $ID set_products '{"address":"0x2", "name":"botas", "price": 450, "stock":10}' --accountId yairnava.testnet
    near call $ID set_products '{"address":"0x3", "name":"tenis", "price": 300, "stock":7}' --accountId yairnava.testnet

Eliminar producto

    near call $ID delete_products '{"address":"0x3"}' --accountId yairnava.testnet