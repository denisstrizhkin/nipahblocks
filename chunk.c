#include "stdlib.h"

#include "chunk.h"

Chunk create_solid_chunk(BlockType* type) {
  Chunk chunk = {};
  chunk.blocks = malloc(CHUNK_SIZE * sizeof(Block));

  for (size_t i = 0; i < CHUNK_SIZE; i++) {
    chunk.blocks[i].type = type;
  }

  return chunk;
}
