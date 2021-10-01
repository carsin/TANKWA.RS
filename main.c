#include "raylib.h"

int main(void) {
  InitWindow(800, 450, "Game --yabai");

  while (!WindowShouldClose()) {
    BeginDrawing();
    ClearBackground(RAYWHITE);
    DrawText("AWWW YEAAA", 190, 200, 20, LIGHTGRAY);
    EndDrawing();
  }
  CloseWindow();
  // fart

  return 0;
}

