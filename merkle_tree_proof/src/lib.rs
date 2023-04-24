use ethers_rs::keccak256;

type Data = str;
type Hash = [u8; 32];

pub struct MerkleTree {
    pub nodes: Vec<Hash>,
}

impl MerkleTree {
    //this is partial implementation of merkle tree
    // pub fn from_hashes(hashes:Vec<&str>) -> Self {
    //     let mut nodes_str = hashes.clone();
    //     let mut level_tree = nodes_str.len();
    //     let mut nodes: Vec<Hash>= Vec::with_capacity(level_tree);
    //     while level_tree > 1 {
    //         let mut level_hashes = Vec::with_capacity((level_tree + 1)/2);
    //         for i in (0..level_tree).step_by(2) {
    //             let hash1 = &nodes_str[i];
    //             let hash2 = if i + 1 < level_tree {
    //                 &nodes_str[i + 1]
    //             } else {
    //                 hash1
    //             };
    //             let combined_hash = &[hash1.as_bytes(), hash2.as_bytes()].concat();
    //             let result = keccak256(combined_hash);
    //             level_hashes.push(result);
    //         }

    //         nodes = level_hashes.clone();
    //         level_tree = level_hashes.len();
    //     }

    //     MerkleTree { nodes }
    // }
    //Full implementation of merkle tree
    pub fn from_hashes(data: Vec<&str>) -> Self {
        let hashes: Vec<Hash> = data.iter().map(|d| keccak256(d.as_bytes())).collect();
        let mut nodes: Vec<[u8; 32]> = hashes.clone();
        let mut level_size: usize = nodes.len();
        while level_size > 1 {
            let mut level_hashes: Vec<[u8; 32]> = Vec::with_capacity((level_size + 1) / 2);
            for i in (0..level_size).step_by(2) {
                let hash1: [u8; 32] = nodes[i];
                let hash2: [u8; 32] = if i + 1 < level_size {
                    nodes[i + 1]
                } else {
                    hash1
                };
                let combined_hash: &Vec<u8> = &[hash1, hash2].concat();
                let result = keccak256(&combined_hash);
                level_hashes.push(result);
            }
            nodes = level_hashes.clone();
            level_size = nodes.len();
        }
        MerkleTree { nodes }
    }

    pub fn get_root(&self) -> Hash {
        *self.nodes.first().unwrap_or(&[0u8; 32])
    }
    
}

