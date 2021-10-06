#include "player.h"
#include "raylib.h"
#include "raymath.h"
#include "stdio.h"

int main(void) {
    // Initialization
    const int screenWidth = 1200;
    const int screenHeight = 800;

    Player player = {};
    player.pos = (Vector2){400, 280};
    player.vel = (Vector2){0.0, 0.0};
    player.speed = 0;
    player.canJump = false;
    EnvItem envItems[] = {{{0, 400, 1000, 200}, true, GRAY},
                          {{300, 200, 400, 10}, true, GRAY},
                          {{250, 300, 100, 10}, true, GRAY},
                          {{650, 300, 100, 10}, true, GRAY}};
    int envItemsLength = sizeof(envItems) / sizeof(envItems[0]);
    Camera2D camera = {};
    camera.target = player.pos;
    camera.offset = (Vector2){screenWidth / 2.0f, screenHeight / 2.0f};
    camera.rotation = 0.0f;
    camera.zoom = 1.0f;

    InitWindow(screenWidth, screenHeight, "Game --yabai");
    SetTargetFPS(144);

    // Main game loop
    while (!WindowShouldClose()) {
        // Update
        float deltaTime = GetFrameTime();
        UpdatePlayer(&player, envItems, envItemsLength, deltaTime);
        UpdatePlayerCamera(&camera, &player, deltaTime, screenWidth, screenHeight);

        // Draw
        BeginDrawing();
        ClearBackground(BLACK);
        DrawFPS(0, 0);
        BeginMode2D(camera);

        for (int i = 0; i < envItemsLength; i++) {
            DrawRectangleRec(envItems[i].rect, envItems[i].color);
        }

        Rectangle playerRect = {player.pos.x - 20, player.pos.y - 40,
                                40, 40};
        DrawRectangleRec(playerRect, WHITE);
        EndMode2D();
        EndDrawing();
    }
    CloseWindow(); // Close window and OpenGL context
    return 0;
}

