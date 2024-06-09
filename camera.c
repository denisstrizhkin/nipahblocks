#include "camera.h"
#include <raylib.h>
#include <raymath.h>

static const float MOUSE_SENS = 0.10f;
static const float CAMERA_SPEED = 10.0f;

static const KeyboardKey KB_FORWARD = KEY_I;
static const KeyboardKey KB_BACKWARD = KEY_K;
static const KeyboardKey KB_LEFT = KEY_J;
static const KeyboardKey KB_RIGHT = KEY_L;

Vector3 get_camera_up(Camera *camera) { return Vector3Normalize(camera->up); }

Vector3 get_camera_forward(Camera *camera) {
  return Vector3Normalize(Vector3Subtract(camera->target, camera->position));
};

Vector3 get_camera_right(Camera *camera) {
  Vector3 forward = get_camera_forward(camera);
  Vector3 up = get_camera_up(camera);
  return Vector3Normalize(Vector3CrossProduct(forward, up));
}


void camera_yaw(Camera *camera, float angle) {
  Vector3 forward = get_camera_forward(camera);
  Vector3 up = get_camera_up(camera);
  Vector3 forward_rotated = Vector3RotateByAxisAngle(forward, up, angle);
  camera->position = Vector3Subtract(camera->target, forward_rotated);
}

void camera_pitch(Camera *camera, float angle) {
  Vector3 forward = get_camera_forward(camera);
  Vector3 right = get_camera_right(camera);
  Vector3 forward_rotated = Vector3RotateByAxisAngle(forward, right, angle);
  camera->position = Vector3Subtract(camera->target, forward_rotated);
}

void camera_move_right(Camera *camera, float distance) {
  Vector3 right = get_camera_right(camera);
  right.y = 0;
  right = Vector3Normalize(right);
  right = Vector3Scale(right, distance);
  camera->target = Vector3Add(camera->target, right);
  camera->position = Vector3Add(camera->position, right);
}

void camera_move_forward(Camera *camera, float distance) {
  Vector3 forward = get_camera_forward(camera);
  forward.y = 0;
  forward = Vector3Normalize(forward);
  forward = Vector3Scale(forward, distance);
  camera->position = Vector3Add(camera->position, forward);
  camera->target = Vector3Add(camera->target, forward);
}

void update_camera(Camera *camera) {
  Vector2 mouse_pos_delta = GetMouseDelta();
  float mouse_wheel = GetMouseWheelMove();

  float dt = GetFrameTime();

  if (IsKeyDown(KB_FORWARD))
    camera_move_forward(camera, CAMERA_SPEED * dt);
  if (IsKeyDown(KB_BACKWARD))
    camera_move_forward(camera, -CAMERA_SPEED * dt);
  if (IsKeyDown(KB_LEFT))
    camera_move_right(camera, -CAMERA_SPEED * dt);
  if (IsKeyDown(KB_RIGHT))
    camera_move_right(camera, CAMERA_SPEED * dt);

  camera_yaw(camera, -mouse_pos_delta.x * dt * MOUSE_SENS);
  camera_pitch(camera, -mouse_pos_delta.y * dt * MOUSE_SENS);
}

Camera create_camera(void) {
  Camera3D camera = {.position = (Vector3){10.0f, 10.0f, 10.0f},
                     .target = (Vector3){0.0f, 0.0f, 0.0f},
                     .up = (Vector3){0.0f, 1.0f, 0.0f},
                     .fovy = 45.0f,
                     .projection = CAMERA_PERSPECTIVE};
  return camera;
}
