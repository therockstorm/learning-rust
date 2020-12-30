#[cxx::bridge(namespace = "org::blobstore")]
mod ffi {
    // Shared structs with fields visible to both languages
    struct BlobMetadata {
        size: usize,
        tags: Vec<String>,
    }

    // Rust types and signatures exposed to C++
    extern "Rust" {
        type MultiBuf;

        fn next_chunk(buf: &mut MultiBuf) -> &[u8];
    }

    // C++ types and signatures exposed to Rust
    unsafe extern "C++" {
        include!("cpp-interop/include/blobstore.h");

        type BlobstoreClient;

        fn new_blobstore_client() -> UniquePtr<BlobstoreClient>;
        fn put(&self, parts: &mut MultiBuf) -> u64;
        fn tag(&self, blobid: u64, tag: &str);
        fn metadata(&self, blobid: u64) -> BlobMetadata;
    }
}

// Iterator over contiguous chunks of discontiguous file object.
//
// Toy impl uses Vec<Vec<u8>>. In reality, might be iterating over complex Rust data 
// structure like rope or lazy loading chunks from somewhere
pub struct MultiBuf {
    chunks: Vec<Vec<u8>>,
    pos: usize,
}
pub fn next_chunk(buf: &mut MultiBuf) -> &[u8] {
    let next = buf.chunks.get(buf.pos);
    buf.pos += 1;
    next.map(Vec::as_slice).unwrap_or(&[])
}

fn main() {
    let client = ffi::new_blobstore_client();

    let chunks = vec![b"fearless".to_vec(), b"concurrency".to_vec()];
    let mut buf = MultiBuf { chunks, pos: 0 };
    let blobid = client.put(&mut buf);
    println!("blobid = {}", blobid);

    client.tag(blobid, "learning rust");

    let metadata = client.metadata(blobid);
    println!("tags = {:?}", metadata.tags);
}
