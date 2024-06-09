#include "raylib.h"
#include "raymath.h"
#include "stdbool.h"
#include "stdio.h"

#include "camera.h"

typedef struct GameInfo {
  bool is_cursor_capture;
} GameInfo;


void draw_cross(void) {
    const int width = GetScreenWidth();
    const int height = GetScreenHeight();
    static const int length = 8;

    Vector2 horizontal_pos = { width / 2 - length / 2, height / 2 - 1};
    Vector2 horizontal_size = { length, 2 };

    Vector2 vertical_pos = { width / 2 - 1, height / 2 - length / 2};
    Vector2 vertical_size = { 2, length };

    DrawRectangleV(horizontal_pos, horizontal_size, BLACK);
    DrawRectangleV(vertical_pos, vertical_size, BLACK);
}

void update_cursor_capture(GameInfo *game_info) {
  if (game_info->is_cursor_capture == false) {
    if (IsMouseButtonPressed(MOUSE_BUTTON_LEFT)) {
      game_info->is_cursor_capture = true;
      DisableCursor();
    }
  } else {
    if (IsKeyPressed(KEY_ESCAPE)) {
      game_info->is_cursor_capture = false;
      EnableCursor();
    }
  }
}

int main(void) {
  const int screenWidth = 800;
  const int screenHeight = 600;
  Camera camera = create_camera();
  GameInfo game_info = {};

  InitWindow(screenWidth, screenHeight,
             "raylib [core] example - 3d camera free");
  SetExitKey(KEY_NULL);

  Vector3 block_pos = {0.0f, 0.0f, 0.0f};
  Vector3 block_size = {1.0f, 1.0f, 1.0f};

  SetTargetFPS(60);
  while (!WindowShouldClose()) {
    update_camera(&camera);
    update_cursor_capture(&game_info);

    BeginDrawing();
    ClearBackground(RAYWHITE);

    BeginMode3D(camera);
    DrawCubeV(block_pos, block_size, RED);
    EndMode3D();

    DrawText(TextFormat("Position: { %.3f, %.3f, %.3f }", camera.position.x,
                        camera.position.y, camera.position.z),
             5, 5, 20, BLACK);
    DrawText(TextFormat("Target: { %.3f, %.3f, %.3f }", camera.target.x,
                        camera.target.y, camera.target.z),
             5, 25, 20, BLACK);
    DrawText(TextFormat("FPS: %d", GetFPS()), 5, 45, 20, BLACK);
    draw_cross();

    EndDrawing();
  }

  CloseWindow();

  return 0;
}
