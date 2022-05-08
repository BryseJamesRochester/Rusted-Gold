struct Transaction {
    from:String,
    nonce:u128,
    pubkey:String,
    sig:Signature,
    outputs:Vec<(String, String)>,
    fee: u32
}

impl Transaction {

}