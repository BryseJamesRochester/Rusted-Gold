struct Transaction {
    from:String,
    nonce:usize,
    pubkey:String,
    sig:String,
    outputs:Vec<(String, String)>,
    fee: u128
}

impl Transaction {

}