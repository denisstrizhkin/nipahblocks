#ifndef CHUNK_H
#define CHUNK_H 1

#include "stdint.h"

#include "raylib.h"
#include "raymath.h"

#include "block.h"

static const uint16_t CHUNK_WIDTH = 16;
static const uint16_t CHUNK_LENGTH = 16;
static const uint16_t CHUNK_HEIGHT = 16;
static const uint16_t CHUNK_SIZE = CHUNK_WIDTH * CHUNK_LENGTH * CHUNK_HEIGHT;

typedef struct Chunk {
  Block* blocks;
} Chunk;

Chunk create_solid_chunk(BlockType*);

#endif
