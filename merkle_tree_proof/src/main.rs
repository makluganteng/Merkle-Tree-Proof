use ethers_rs::keccak256;
use merkle_tree_proof::MerkleTree;

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


    
}
