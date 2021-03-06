## About

Texture Block Compression (BCn) written in Rust. Block compression is used to compress textures for GPU, 
there are lots of variations BC1 (DXT1), BC3 (DXT5), and so on. Compressed textures has much
lower requirements for memory bandwidth and especially useful for slow memory used by built-in GPUs.
Almost every GPUs starting from 1998 has **hardware** decompressor for compressed textures, so there is
no performance penalty of compression.

## Supported formats

- BC1 (DXT1)
- BC3 (DXT5)
- BC4 (Both R8 and RG8)

## References

- [Block compression guide from Microsoft](https://docs.microsoft.com/en-us/windows/win32/direct3d10/d3d10-graphics-programming-guide-resources-block-compression)
- [Real-Time DXT Compression by J.M.P. Van Waveren](https://www.researchgate.net/publication/259000525_Real-Time_DXT_Compression)
- [Understanding BCn Texture Compression Formats](https://www.reedbeta.com/blog/understanding-bcn-texture-compression-formats/)
- [OpenGL Data Format Specification](https://www.khronos.org/registry/DataFormat/specs/1.3/dataformat.1.3.html)