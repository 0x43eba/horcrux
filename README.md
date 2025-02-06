## Horcrux

This is a small project I created to both learn Rust, and store encrypted data in multiple redundant shards on IPFS. 
The algorithm is not particularily practical, since you require control or access of multiple IPFS nodes for it to be effective. 
That said, it was an interesting challenge to build.

### Process Flow

This is the general flow of creating the shards, and encrypting them. After this process is completed, they are then forwarded to IPFS as Base64 encoded fragments.

<img width="733" alt="Screenshot 2024-03-07 at 01 36 08" src="https://github.com/mapleman-is/horcrux/assets/76260172/ba79b219-2e33-4cb8-bf92-dc9178f7ecaa">

Once transmitted, the file locations are returned as hashes. I also pin the files to the specific node they were uploaded to. The implementation assumes you have a local IPFS
node running at the standard address.

<img width="526" alt="Screenshot 2024-03-07 at 01 38 20" src="https://github.com/mapleman-is/horcrux/assets/76260172/1fcb66e8-af18-4ede-b9c1-b997bc062510">

This is where the cool in a James Bond-esque way, but not very practical part of the project comes into play. Alpha Channel Steganography. Since the algorithm emits a large number
of hashes (1 for every shard times n redundant shards), I needed a place to keep them, and a way to cleanly send bulk hash data to other node operators for pinning. I embed the hashes
in the Alpha Channel of the image, and output a visually identical output image with this data embedded.

When this file is passed to the CLI, the tool extracts the hashes from the Alpha Channel, and executes a pin operation against the node the tool is pointed at. 
This means you can ask a maintainer to pin your shards, simply by sending them a photograph.

### Known Issues

There's lots of redundant Base64 encoding/decoding steps left over from early testing that need to be cleared out. Ultimately we only need one on the pre-encryption step, and one
when it is packaged for transmission.
