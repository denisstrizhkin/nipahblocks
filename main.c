#include "raylib.h"
#include "raymath.h"
#include "stdio.h"
#include "stdbool.h"

#include "camera.h"

typedef struct Region {
  Vector3 low;
  Vector3 high;
  bool is_wires;
  Color color;
} Region;

void draw_region(Region *region) {
  Vector3 pos = Vector3Scale(Vector3Add(region->high, region->low), 0.5f);
  Vector3 size = Vector3Subtract(region->high, region->low);

  if (region->is_wires) {
    DrawCubeWiresV(pos, size, region->color);
  } else {
    DrawCubeV(pos, size, ColorAlpha(region->color, 0.75f));
  }
}


int main(void)
{
    // Initialization
    //--------------------------------------------------------------------------------------
    const int screenWidth = 800;
    const int screenHeight = 600;
    Camera camera = create_camera();

    InitWindow(screenWidth, screenHeight, "raylib [core] example - 3d camera free");

    Vector3 block_pos = {0.0f, 0.0f, 0.0f};
    Vector3 block_size = {1.0f, 1.0f, 1.0f};

    SetTargetFPS(60);
    while (!WindowShouldClose())
    {
      update_camera(&camera);

      if (IsKeyPressed('Z')) camera.target = (Vector3){ 0.0f, 0.0f, 0.0f };
      BeginDrawing();
        ClearBackground(RAYWHITE);

	BeginMode3D(camera);
	DrawCubeV(block_pos, block_size, RED);
	EndMode3D();

       
      EndDrawing();
    }

    CloseWindow();

    return 0;
}
