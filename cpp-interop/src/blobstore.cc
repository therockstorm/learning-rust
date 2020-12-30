#include "cpp-interop/include/blobstore.h"
#include "cpp-interop/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <unordered_map>

namespace org {
  namespace blobstore {
    // Toy impl of an in-memory blobstore
    // In reality, impl could be complex C++ library
    class BlobstoreClient::impl {
      friend BlobstoreClient;
      using Blob = struct {
        std::string data;
        std::set<std::string> tags;
      };
      std::unordered_map<uint64_t, Blob> blobs;
    };

    BlobstoreClient::BlobstoreClient() : impl(new class BlobstoreClient::impl) {}

    // Upload new blob and return ID
    uint64_t BlobstoreClient::put(MultiBuf &buf) const {
      std::string contents;

      // Traverse caller's chunk iterator
      // In reality, might be sophisticated chunk batching and/or parallel upload
      while (true) {
        auto chunk = next_chunk(buf);
        if (chunk.size() == 0) {
          break;
        }
        contents.append(reinterpret_cast<const char *>(chunk.data()), chunk.size());
      }

      // Insert into map and provide caller handle
      auto blobid = std::hash<std::string>{}(contents);
      impl->blobs[blobid] = {std::move(contents), {}};
      return blobid;
    }

    // Add tag to blob
    void BlobstoreClient::tag(uint64_t blobid, rust::Str tag) const {
      impl->blobs[blobid].tags.emplace(tag);
    }

    // Retrieve blob metadata
    BlobMetadata BlobstoreClient::metadata(uint64_t blobid) const {
      BlobMetadata metadata{};
      auto blob = impl->blobs.find(blobid);
      if (blob != impl->blobs.end()) {
        metadata.size = blob->second.data.size();
        std::for_each(blob->second.tags.cbegin(), blob->second.tags.cend(),
                      [&](auto &t) { metadata.tags.emplace_back(t); });
      }
      return metadata;
    }

    std::unique_ptr<BlobstoreClient> new_blobstore_client() {
      return std::make_unique<BlobstoreClient>();
    }
  }
}
