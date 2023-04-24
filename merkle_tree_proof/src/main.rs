use std::{fs::{File, self}, io::BufReader};
use serde::{Deserialize, Serialize};
use ethers_rs::keccak256;
use merkle_tree_proof::MerkleTree;
#[derive(Debug, Deserialize, Serialize)]
struct Transaction {
    from: String,
    to: String,
    gas_limit: String,
    max_fee_per_gas: String,
    max_priority_fee_per_gas: String,
    nonce: String,
    value: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct Block {
    block_number: u64,
    timestamp: u64,
    transactions: Vec<Transaction>,
}

fn main() {
    //simple implementation of keccak the data
    let data1 = "hello";
    let data2 = "world";
    let data3 = "foo";
    let data4 = "bar";


    let data1_byte = data1.as_bytes();
    let data2_byte = data2.as_bytes();
    // let result = keccak256(data1_byte);
    // let result1 = keccak256(data2_byte);

   let result = &[data1_byte, data2_byte].concat();
    let result_keccak = keccak256(result);
    let result_hex = hex::encode(result_keccak);
    println!("data1: {:?}, result: {:?}", data1_byte, result_hex);
    
    //create a merkle tree from the data
    let datas = vec![data1, data2, data3, data4];
    let merkle_tree = MerkleTree::from_hashes(datas);
    let root = MerkleTree::get_root(&merkle_tree);
    let root_hex = hex::encode(root);
    println!("root: {:?}", root_hex);

    let json = r#"
    [
        {
            "block_number": 7890,
            "timestamp": 1630768715,
            "transactions": [
                {
                    "from": "0x23B87274aB2B0b484Cd0Bf26135688d9c137F64F",
                    "to": "0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D",
                    "gas_limit": "75000",
                    "max_fee_per_gas": "20",
                    "max_priority_fee_per_gas": "2",
                    "nonce": "17",
                    "value": "1000000000000000000"
                }
            ]
        },
        {
            "block_number": 12345,
            "timestamp": 1618241746,
            "transactions": [
                {
                    "from": "0x2c10cE3454C4F15D742Bd61aB68faA0238f0B9c9",
                    "to": "0x1cEB5cB57C4D4E2b2433641b95Dd330A33185A44",
                    "gas_limit": "100000",
                    "max_fee_per_gas": "25",
                    "max_priority_fee_per_gas": "1",
                    "nonce": "25",
                    "value": "500000000000000000"
                }
            ]
        },
        {
            "block_number": 34567,
            "timestamp": 1644388690,
            "transactions": [
                {
                    "from": "0x345c2aEeeFDf45cF8fF1bEd2cB2Ccc123456789",
                    "to": "0xD26a3A5b37c1FA5d00BdC1e45A10F9c123456789",
                    "gas_limit": "50000",
                    "max_fee_per_gas": "40",
                    "max_priority_fee_per_gas": "5",
                    "nonce": "10",
                    "value": "250000000000000000"
                }
            ]
        },
        {
            "block_number": 45678,
            "timestamp": 1662870800,
            "transactions": [
                {
                    "from": "0x9012cA34A289b75c0AbcDeF1dCFe8Dc123456789",
                    "to": "0x1234567890AbcDEF1234567890abcDEF12345678",
                    "gas_limit": "80000",
                    "max_fee_per_gas": "50",
                    "max_priority_fee_per_gas": "10",
                    "nonce": "12",
                    "value": "1000000000000000000"
                }
            ]
        }    
    ]
    "#;
    
    let block: Vec<Block> = serde_json::from_str(json).unwrap();
    println!("{:?}", block);
    // let file = File::open("").unwrap();
    // let buffer_read = BufReader::new(file);
    
    // let block: Block = serde_json::from_reader(buffer_read).unwrap();

    // let file:Block = {
    // let file_content = fs::read_to_string("./example.json").expect("Cant read JSON file");
    // serde_json::from_str(&file_content).expect("Cant parse JSON file")
    // };

    // println!("{:?}", serde_json::to_string_pretty(&file).expect("LogRocket: error parsing to JSON"));

    for blocks in block {
        println!("{:?}", blocks.block_number);
        println!("{:?}", blocks.timestamp);
        for transactions in blocks.transactions {
            println!("{:?}", transactions.from);
            println!("{:?}", transactions.to);
            println!("{:?}", transactions.gas_limit);
            println!("{:?}", transactions.max_fee_per_gas);
            println!("{:?}", transactions.max_priority_fee_per_gas);
            println!("{:?}", transactions.nonce);
            println!("{:?}", transactions.value);
        }
    }
    
}
