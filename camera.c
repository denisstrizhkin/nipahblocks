#include "camera.h"
#include <raylib.h>

static const float MOUSE_SENS = 0.25f;
static const float CAMERA_SPEED = 10.0f;
static const float ZOOM_SPEED = 100.0f;

static const KeyboardKey KB_FORWARD = KEY_I;
static const KeyboardKey KB_BACKWARD = KEY_K;
static const KeyboardKey KB_LEFT = KEY_J;
static const KeyboardKey KB_RIGHT = KEY_L;

Vector3 get_camera_right(Camera *camera) {
  Vector3 z_forward = Vector3Subtract(camera->target, camera->position);
  return Vector3Normalize(Vector3CrossProduct(z_forward, camera->up));
}

Vector3 get_camera_up(Camera *camera) {
  return Vector3Normalize(camera->up);
}

void camera_yaw(Camera *camera, float angle) {
  Vector3 z_neg = Vector3Subtract(camera->target, camera->position);
  Vector3 up = get_camera_up(camera);
  Vector3 z_neg_rotated = Vector3RotateByAxisAngle(z_neg, up, angle); 
  camera->position = Vector3Subtract(camera->target, z_neg_rotated);
}

void camera_pitch(Camera *camera, float angle) {
  Vector3 z_neg = Vector3Subtract(camera->target, camera->position);
  Vector3 right = get_camera_right(camera);
  Vector3 z_neg_rotated = Vector3RotateByAxisAngle(z_neg, right, angle);
  camera->position = Vector3Subtract(camera->target, z_neg_rotated);
}

void camera_up(Camera *camera, float distance) {
  Vector3 up = get_camera_up(camera);
  up = Vector3Scale(up, distance);
  camera->target = Vector3Add(camera->target, up);
  camera->position = Vector3Add(camera->position, up);
}

void camera_right(Camera *camera, float distance) {
  Vector3 right = get_camera_right(camera);
  right = Vector3Scale(right, distance);
  camera->target = Vector3Add(camera->target, right);
  camera->position = Vector3Add(camera->position, right);
}

void update_camera(Camera *camera) {
  Vector2 mouse_pos_delta = GetMouseDelta();
  float mouse_wheel = GetMouseWheelMove();
  
  float dt = GetFrameTime();
  
  if (IsMouseButtonDown(MOUSE_BUTTON_LEFT)) {
    camera_yaw(camera, -mouse_pos_delta.x * MOUSE_SENS * dt);
    camera_pitch(camera, -mouse_pos_delta.y * MOUSE_SENS * dt);
  }

  camera->fovy += mouse_wheel * ZOOM_SPEED * dt;
  if (camera->fovy < 0.001f) camera->fovy = 0.001f;

  if (IsKeyDown(KB_FORWARD)) camera_up(camera, CAMERA_SPEED * dt); 
  if (IsKeyDown(KB_BACKWARD)) camera_up(camera, -CAMERA_SPEED * dt);
  if (IsKeyDown(KB_LEFT)) camera_right(camera, -CAMERA_SPEED * dt);
  if (IsKeyDown(KB_RIGHT)) camera_right(camera, CAMERA_SPEED * dt);

  if (IsMouseButtonDown(MOUSE_BUTTON_MIDDLE)) {
    camera_up(camera, mouse_pos_delta.y * MOUSE_SENS * CAMERA_SPEED * dt);
    camera_right(camera, -mouse_pos_delta.x * MOUSE_SENS * CAMERA_SPEED * dt);
  }
}

Camera create_camera(void) {
    Camera3D camera = { 
      .position = (Vector3){ 10.0f, 10.0f, 10.0f },
      .target = (Vector3){ 0.0f, 0.0f, 0.0f },
      .up = (Vector3){ 0.0f, 1.0f, 0.0f },
      .fovy = 45.0f,
      .projection = CAMERA_PERSPECTIVE
    };
    return camera;
}
