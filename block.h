#ifndef BLOCK_H
#define BLOCK_H 1
#include "raylib.h"
#include "raymath.h"
#include "stdbool.h"

typedef struct BlockType {
  const char * name;
  bool is_solid;
  int texture_top;
  int texture_front;
  int texture_right;
  int texture_left;
  int texture_back;
  int texture_bottom;
} BlockType;

static const BlockType block_dirt = {
  .name = "dirt",
  .is_solid = true,
  .texture_top = NULL,
  .texture_front = NULL,
  .texture_right = NULL,
  .texture_left = NULL,
  .texture_back = NULL,
  .texture_bottom = NULL
};

typedef struct Block {
  const BlockType* type;
} Block;

#endif
